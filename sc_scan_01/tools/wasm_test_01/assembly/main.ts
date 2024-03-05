import {
    generateEvent,
    Context,
} from "@massalabs/massa-as-sdk";

import {
    Args,
    stringToBytes,
} from "@massalabs/as-types";

import {
    setOwner,
} from '@massalabs/sc-standards/assembly/contracts/utils/ownership';

const data: StaticArray<u8> = stringToBytes("42");

export function constructor(args: StaticArray<u8>): void {
    assert(Context.isDeployingContract());
    setOwner(new Args().add(Context.caller()).serialize());
}

export function main(): void {
    generateEvent(`wasm_test_01 - ${data}`);
}

