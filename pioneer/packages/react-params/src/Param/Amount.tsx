// Copyright 2017-2019 @polkadot/react-components authors & contributors
// This software may be modified and distributed under the terms
// of the Apache-2.0 license. See the LICENSE file for details.

import { Props } from '../types';

import BN from 'bn.js';
import React from 'react';
// import { ClassOf } from '@polkadot/types';
import { Input } from '@polkadot/react-components';
// import { bnToBn, formatNumber } from '@polkadot/util';

import Bare from './Bare';

function onChange ({ onChange }: Props): (_: string) => void {
  return function (value: string): void {
    onChange && onChange({
      isValid: !!value,
      value: new BN(value || 0)
    });
  };
}

export default function Amount (props: Props): React.ReactElement<Props> {
  const { className, defaultValue: { value }, isDisabled, isError, label, onEnter, style, withLabel } = props;

  const defaultValue = value ? value.toString() : '0';

  return (
    <Bare
      className={className}
      style={style}
    >
      <Input
        className='full'
        defaultValue={defaultValue}
        isDisabled={isDisabled}
        isError={isError}
        label={label}
        min={0}
        onChange={onChange(props)}
        onEnter={onEnter}
        type={
          isDisabled
            ? 'text'
            : 'number'
        }
        withEllipsis
        withLabel={withLabel}
      />
    </Bare>
  );
}
