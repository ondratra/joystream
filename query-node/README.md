# query-node

The query-node project contains an input schema (schema.graphql) and mappings for the Joystream `content-directory` runtime module.

## Code generation

We use Hydra-cli to generate a graphql server and a block indexer for joystream chain:

```bash
$ cd query-node
$ yarn build
```

**Note:** For more information about how to create new mappings and general info about Hydra,
see https://github.com/Joystream/hydra/tree/master/examples/joystream-query-node
and https://github.com/Joystream/hydra/tree/master/packages/sample    .

## Starting services

To start services defined in the project docker-compose.yml, you should run docker-compose from the project root folder to use the correct .env file

## Run mapping processor

Before running mappings make sure indexer(`yarn indexer:start`) and indexer-api-server (mappings get the chain data from this graphql server) are both running:

```bash
yarn processor:start
```

## Query data

Once processor start to store event data you will be able to query this data from `http://localhost:4002/graphql`.

```graphql
query {
  channels {
    handle
  }
}
```

## Dev workflow - old

Setup the development environment:
```
# optional - clear the environment
docker stop $(docker ps -aq) # make sure all docker containers are stopped (Warning: this command stops ALL containers on your machine!)
docker-compose down -v # remove all docker volumes
rm query-node/generated -r # remove any previously generated query-node code

# services setup
yarn workspace query-node-root build # install dependencies and build query node
docker-compose up -d db # start db container

docker-compose up -d joystream-node # start node
yarn workspace query-node-root gen-types # generate typescript types
yarn workspace query-node-root db:migrate # create db from schema

docker-compose up -d redis # start caching db
docker-compose up -d indexer-api-gateway # startup api gateway
docker stop joystream_indexer_1 # turn off indexer running in docker (started as dependency in the previous command)

# run indexer in the first terminal
yarn workspace query-node-root indexer:start # start indexer

# run processor in the second terminal
DEBUG=* yarn workspace query-node-root processor:start

# continue in the third terminal
yarn workspace @joystream/cd-schemas initialize:dev # initialize Joystream schemas
yarn workspace @joystream/cd-schemas example:createChannel # create example channel - this change will take effect in ~90 sec
yarn workspace query-node-root server:start:dev # start graphql server
```

Depending on the changes you are making, you might want to delete folder `query-node/node_modules/@joystream/types`
and replace it with a symlink targeting your local folder `ln -s ../../../types types`.

How to make change in input schema:
```
# first make a change to schema.graphql
# stop the indexer instance

docker-compose up -V -d db # start new clear db
rm query-node/generated -r # remove any previously generated query-node code
yarn workspace query-node-root build
yarn workspace query-node-root db:migrate # recreate db

# restart indexer
yarn workspace query-node-root indexer:start # start indexer
```

tmp
DB_NAME=query_node_indexer COMPOSE_PROJECT_NAME=joystream PROJECT_NAME=query_node INDEXER_DB_NAME=query_node_indexer PROCESSOR_DB_NAME=query_node_processor DB_USER=postgres DB_PASS=postgres DB_HOST=localhost DB_PORT=5432 DEBUG=index-builder:* TYPEORM_LOGGING=error WS_PROVIDER_ENDPOINT_URI=ws://localhost:9944/ BLOCK_HEIGHT=0 REDIS_URI=redis://localhost:6379/0 TYPES_JSON=../../../types/augment/all/defs.json INDEXER_ENDPOINT_URL=http://localhost:4000/graphql BLOCK_HEIGHT=0 GRAPHQL_SERVER_PORT=4002 GRAPHQL_SERVER_HOST=localhost WARTHOG_APP_PORT=4002 WARTHOG_APP_HOST=localhost yarn workspace query-node-root newcodegen2


So Andy just a quick recap:
1 - The content-directory-schemas package now is completely irrelevant for sumer
2 - the types packages version in the monorepo for sumer release has been bumped from v0.14.0 to v0.16.0 (v0.15.0 was assigned in olympia branch) so you have to make sure to use that version in query-node/integration tests/cli to use the workspace version rather than the one published on npm
3 - https://polkadot.js.org/docs/ to understand how interact with the chain but of course you can see alot of example code in our cli and network tests code (plus the code snippets in utils/api-scripts)
polkadot.js.org
Overview | polkadot{.js}
Got here looking for the Apps UI (Wallet)? Just follow the preceding link. Looking for developer documentation? Then you are at the right place.

## Things regarding the hydra

- copy all "db:" to query node (look into bootstrap) 
- create package.json in the query-node/mappings similar to sample project
- run processor:start
- copy manifest.yml

-run indexer before processer

- usefull info in hydra-e2e-tests/docker-compose.yml
- example tests in hydra-e2e-tests/run-tests.sh
