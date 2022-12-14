/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/
import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { Adr36Info, AddressResponse, AddressByIcnsResponse, AddressesResponse, AdminResponse, Config, IcnsNamesResponse, NamesResponse, PrimaryNameResponse } from "./IcnsResolver.types";
export interface IcnsResolverReadOnlyInterface {
    contractAddress: string;
    config: () => Promise<Config>;
    addresses: ({ name }: {
        name: string;
    }) => Promise<AddressesResponse>;
    address: ({ bech32Prefix, name }: {
        bech32Prefix: string;
        name: string;
    }) => Promise<AddressResponse>;
    names: ({ address }: {
        address: string;
    }) => Promise<NamesResponse>;
    icnsNames: ({ address }: {
        address: string;
    }) => Promise<IcnsNamesResponse>;
    primaryName: ({ address }: {
        address: string;
    }) => Promise<PrimaryNameResponse>;
    admin: () => Promise<AdminResponse>;
    addressByIcns: ({ icns }: {
        icns: string;
    }) => Promise<AddressByIcnsResponse>;
}
export declare class IcnsResolverQueryClient implements IcnsResolverReadOnlyInterface {
    client: CosmWasmClient;
    contractAddress: string;
    constructor(client: CosmWasmClient, contractAddress: string);
    config: () => Promise<Config>;
    addresses: ({ name }: {
        name: string;
    }) => Promise<AddressesResponse>;
    address: ({ bech32Prefix, name }: {
        bech32Prefix: string;
        name: string;
    }) => Promise<AddressResponse>;
    names: ({ address }: {
        address: string;
    }) => Promise<NamesResponse>;
    icnsNames: ({ address }: {
        address: string;
    }) => Promise<IcnsNamesResponse>;
    primaryName: ({ address }: {
        address: string;
    }) => Promise<PrimaryNameResponse>;
    admin: () => Promise<AdminResponse>;
    addressByIcns: ({ icns }: {
        icns: string;
    }) => Promise<AddressByIcnsResponse>;
}
export interface IcnsResolverInterface extends IcnsResolverReadOnlyInterface {
    contractAddress: string;
    sender: string;
    setRecord: ({ adr36Info, bech32Prefix, name }: {
        adr36Info: Adr36Info;
        bech32Prefix: string;
        name: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    setPrimary: ({ bech32Address, name }: {
        bech32Address: string;
        name: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    removeRecord: ({ bech32Address, name }: {
        bech32Address: string;
        name: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export declare class IcnsResolverClient extends IcnsResolverQueryClient implements IcnsResolverInterface {
    client: SigningCosmWasmClient;
    sender: string;
    contractAddress: string;
    constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string);
    setRecord: ({ adr36Info, bech32Prefix, name }: {
        adr36Info: Adr36Info;
        bech32Prefix: string;
        name: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    setPrimary: ({ bech32Address, name }: {
        bech32Address: string;
        name: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
    removeRecord: ({ bech32Address, name }: {
        bech32Address: string;
        name: string;
    }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
//# sourceMappingURL=IcnsResolver.client.d.ts.map