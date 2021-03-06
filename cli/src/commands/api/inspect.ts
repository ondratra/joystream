import { flags } from '@oclif/command';
import { CLIError } from '@oclif/errors';
import { displayNameValueTable } from '../../helpers/display';
import { ApiPromise } from '@polkadot/api';
import { getTypeDef } from '@polkadot/types';
import { Codec, TypeDef, TypeDefInfo } from '@polkadot/types/types';
import { ConstantCodec } from '@polkadot/api-metadata/consts/types';
import ExitCodes from '../../ExitCodes';
import chalk from 'chalk';
import { NameValueObj } from '../../Types';
import inquirer from 'inquirer';
import ApiCommandBase from '../../base/ApiCommandBase';

// Command flags type
type ApiInspectFlags = {
    type: string,
    module: string,
    method: string,
    exec: boolean,
    callArgs: string
};

// Currently "inspectable" api types
const TYPES_AVAILABLE = [
    'query',
    'consts',
] as const;

// String literals type based on TYPES_AVAILABLE const.
// It works as if we specified: type ApiType = 'query' | 'consts'...;
type ApiType = typeof TYPES_AVAILABLE[number];

// Format of the api input args (as they are specified in the CLI)
type ApiMethodInputSimpleArg = string;
// This recurring type allows the correct handling of nested types like:
// ((Type1, Type2), Option<Type3>) etc.
type ApiMethodInputArg = ApiMethodInputSimpleArg | ApiMethodInputArg[];

export default class ApiInspect extends ApiCommandBase {
    static description =
        'Lists available node API modules/methods and/or their description(s), '+
        'or calls one of the API methods (depending on provided arguments and flags)';

    static examples = [
        '$ api:inspect',
        '$ api:inspect -t=query',
        '$ api:inspect -t=query -M=members',
        '$ api:inspect -t=query -M=members -m=memberProfile',
        '$ api:inspect -t=query -M=members -m=memberProfile -e',
        '$ api:inspect -t=query -M=members -m=memberProfile -e -a=1',
    ];

    static flags = {
        type: flags.string({
            char: 't',
            description:
                'Specifies the type/category of the inspected request (ie. "query", "consts" etc.).\n'+
                'If no "--module" flag is provided then all available modules in that type will be listed.\n'+
                'If this flag is not provided then all available types will be listed.',
        }),
        module: flags.string({
            char: 'M',
            description:
                'Specifies the api module, ie. "system", "staking" etc.\n'+
                'If no "--method" flag is provided then all methods in that module will be listed along with the descriptions.',
            dependsOn: ['type'],
        }),
        method: flags.string({
            char: 'm',
            description: 'Specifies the api method to call/describe.',
            dependsOn: ['module'],
        }),
        exec: flags.boolean({
            char: 'e',
            description: 'Provide this flag if you want to execute the actual call, instead of displaying the method description (which is default)',
            dependsOn: ['method'],
        }),
        callArgs: flags.string({
            char: 'a',
            description:
                'Specifies the arguments to use when calling a method. Multiple arguments can be separated with a comma, ie. "-a=arg1,arg2".\n'+
                'You can omit this flag even if the method requires some aguments.\n'+
                'In that case you will be promted to provide value for each required argument.\n' +
                'Ommiting this flag is recommended when input parameters are of more complex types (and it\'s hard to specify them as just simple comma-separated strings)',
            dependsOn: ['exec'],
        })
    };

    getMethodMeta(apiType: ApiType, apiModule: string, apiMethod: string) {
        if (apiType === 'query') {
            return this.getOriginalApi().query[apiModule][apiMethod].creator.meta;
        }
        else {
            // Currently the only other optoin is api.consts
            const method:ConstantCodec = <ConstantCodec> this.getOriginalApi().consts[apiModule][apiMethod];
            return method.meta;
        }
    }

    getMethodDescription(apiType: ApiType, apiModule: string, apiMethod: string): string {
        let description:string = this.getMethodMeta(apiType, apiModule, apiMethod).documentation.join(' ');
        return description || 'No description available.';
    }

    getQueryMethodParamsTypes(apiModule: string, apiMethod: string): string[] {
        const method = this.getOriginalApi().query[apiModule][apiMethod];
        const { type } = method.creator.meta;
        if (type.isDoubleMap) {
            return [ type.asDoubleMap.key1.toString(), type.asDoubleMap.key2.toString() ];
        }
        if (type.isMap) {
            return type.asMap.linked.isTrue ? [ `Option<${type.asMap.key.toString()}>` ] : [ type.asMap.key.toString() ];
        }
        return [];
    }

    getMethodReturnType(apiType: ApiType, apiModule: string, apiMethod: string): string {
        if (apiType === 'query') {
            const method = this.getOriginalApi().query[apiModule][apiMethod];
            const { meta: { type, modifier } } = method.creator;
            if (type.isDoubleMap) {
                return type.asDoubleMap.value.toString();
            }
            if (modifier.isOptional) {
                return `Option<${type.toString()}>`;
            }
        }
        // Fallback for "query" and default for "consts"
        return this.getMethodMeta(apiType, apiModule, apiMethod).type.toString();
    }

    // Validate the flags - throws an error if flags.type, flags.module or flags.method is invalid / does not exist in the api.
    // Returns type, module and method which validity we can be sure about (notice they may still be "undefined" if weren't provided).
    validateFlags(api: ApiPromise, flags: ApiInspectFlags): { apiType: ApiType | undefined, apiModule: string | undefined, apiMethod: string | undefined } {
        let apiType: ApiType | undefined = undefined;
        const { module: apiModule, method: apiMethod } = flags;

        if (flags.type !== undefined) {
            const availableTypes: readonly string[] = TYPES_AVAILABLE;
            if (!availableTypes.includes(flags.type)) {
                throw new CLIError('Such type is not available', { exit: ExitCodes.InvalidInput });
            }
            apiType = <ApiType> flags.type;
            if (apiModule !== undefined) {
                if (!api[apiType][apiModule]) {
                    throw new CLIError('Such module was not found', { exit: ExitCodes.InvalidInput });
                }
                if (apiMethod !== undefined && !api[apiType][apiModule][apiMethod]) {
                    throw new CLIError('Such method was not found', { exit: ExitCodes.InvalidInput });
                }
            }
        }

        return { apiType, apiModule, apiMethod };
    }

    // Prompt for simple value (string)
    async promptForSimple(typeName: string): Promise<string> {
        const userInput = await inquirer.prompt([{
            name: 'providedValue',
            message: `Provide value for ${ typeName }`,
            type: 'input'
        } ])
        return <string> userInput.providedValue;
    }

    // Prompt for optional value (returns undefined if user refused to provide)
    async promptForOption(typeDef: TypeDef): Promise<ApiMethodInputArg | undefined> {
        const userInput = await inquirer.prompt([{
            name: 'confirmed',
            message: `Do you want to provide the optional ${ typeDef.type } parameter?`,
            type: 'confirm'
        } ]);

        if (userInput.confirmed) {
            const subtype = <TypeDef> typeDef.sub; // We assume that Opion always has a single subtype
            let value = await this.promptForParam(subtype.type);
            return value;
        }
    }

    // Prompt for tuple - returns array of values
    async promptForTuple(typeDef: TypeDef): Promise<(ApiMethodInputArg)[]> {
        let result: ApiMethodInputArg[] = [];

        if (!typeDef.sub) return [ await this.promptForSimple(typeDef.type) ];

        const subtypes: TypeDef[] = Array.isArray(typeDef.sub) ? typeDef.sub : [ typeDef.sub ];

        for (let subtype of subtypes) {
            let inputParam = await this.promptForParam(subtype.type);
            if (inputParam !== undefined) result.push(inputParam);
        }

        return result;
    }

    // Prompt for param based on "paramType" string (ie. Option<MemeberId>)
    async promptForParam(paramType: string): Promise<ApiMethodInputArg | undefined> {
        const typeDef: TypeDef = getTypeDef(paramType);
        if (typeDef.info === TypeDefInfo.Option) return await this.promptForOption(typeDef);
        else if (typeDef.info === TypeDefInfo.Tuple) return await this.promptForTuple(typeDef);
        else return await this.promptForSimple(typeDef.type);
    }

    // Request values for params using array of param types (strings)
    async requestParamsValues(paramTypes: string[]): Promise<ApiMethodInputArg[]> {
        let result: ApiMethodInputArg[] = [];
        for (let [key, paramType] of Object.entries(paramTypes)) {
            this.log(chalk.bold.white(`Parameter no. ${ parseInt(key)+1 } (${ paramType }):`));
            let paramValue = await this.promptForParam(paramType);
            if (paramValue !== undefined) result.push(paramValue);
        }

        return result;
    }

    async run() {
        const api: ApiPromise = this.getOriginalApi();
        const flags: ApiInspectFlags = <ApiInspectFlags> this.parse(ApiInspect).flags;
        const availableTypes: readonly string[] = TYPES_AVAILABLE;
        const { apiType, apiModule, apiMethod } = this.validateFlags(api, flags);

        // Executing a call
        if (apiType && apiModule && apiMethod && flags.exec) {
            let result: Codec;

            if (apiType === 'query') {
                // Api query - call with (or without) arguments
                let args: ApiMethodInputArg[] = flags.callArgs ? flags.callArgs.split(',') : [];
                const paramsTypes: string[] = this.getQueryMethodParamsTypes(apiModule, apiMethod);
                if (args.length < paramsTypes.length) {
                    this.warn('Some parameters are missing! Please, provide the missing parameters:');
                    let missingParamsValues = await this.requestParamsValues(paramsTypes.slice(args.length));
                    args = args.concat(missingParamsValues);
                }
                result = await api.query[apiModule][apiMethod](...args);
            }
            else {
                // Api consts - just assign the value
                result = api.consts[apiModule][apiMethod];
            }

            this.log(chalk.green(result.toString()));
        }
        // Describing a method
        else if (apiType && apiModule && apiMethod) {
            this.log(chalk.bold.white(`${ apiType }.${ apiModule }.${ apiMethod }`));
            const description: string = this.getMethodDescription(apiType, apiModule, apiMethod);
            this.log(`\n${ description }\n`);
            let typesRows: NameValueObj[] = [];
            if (apiType === 'query') {
                typesRows.push({ name: 'Params:', value: this.getQueryMethodParamsTypes(apiModule, apiMethod).join(', ') || '-' });
            }
            typesRows.push({ name: 'Returns:', value: this.getMethodReturnType(apiType, apiModule, apiMethod) });
            displayNameValueTable(typesRows);
        }
        // Displaying all available methods
        else if (apiType && apiModule) {
            const module = api[apiType][apiModule];
            const rows: NameValueObj[] = Object.keys(module).map((key: string) => {
                return { name: key, value: this.getMethodDescription(apiType, apiModule, key) };
            });
            displayNameValueTable(rows);
        }
        // Displaying all available modules
        else if (apiType) {
            this.log(chalk.bold.white('Available modules:'));
            this.log(Object.keys(api[apiType]).map(key => chalk.white(key)).join('\n'));
        }
        // Displaying all available types
        else {
            this.log(chalk.bold.white('Available types:'));
            this.log(availableTypes.map(type => chalk.white(type)).join('\n'));
        }
    }
}
