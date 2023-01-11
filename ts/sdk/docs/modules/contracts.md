[icns](../README.md) / contracts

# Namespace: contracts

## Table of contents

### Variables

- [IcnsNameNft](contracts.md#icnsnamenft)
- [IcnsRegistrar](contracts.md#icnsregistrar)
- [IcnsResolver](contracts.md#icnsresolver)

## Variables

### IcnsNameNft

• `Const` **IcnsNameNft**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `IcnsNameNftClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`IcnsNameNftClient`](contracts.md#icnsnamenftclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `admin`: () => `Promise`<`AdminResponse`\> ; `allNftInfo`: (`__namedParameters`: { `includeExpired?`: `boolean` ; `tokenId`: `string`  }) => `Promise`<`AllNftInfoResponseForMetadata`\> ; `allOperators`: (`__namedParameters`: { `includeExpired?`: `boolean` ; `limit?`: `number` ; `owner`: `string` ; `startAfter?`: `string`  }) => `Promise`<`OperatorsResponse`\> ; `allTokens`: (`__namedParameters`: { `limit?`: `number` ; `startAfter?`: `string`  }) => `Promise`<`TokensResponse`\> ; `approval`: (`__namedParameters`: { `includeExpired?`: `boolean` ; `spender`: `string` ; `tokenId`: `string`  }) => `Promise`<`ApprovalResponse`\> ; `approvals`: (`__namedParameters`: { `includeExpired?`: `boolean` ; `tokenId`: `string`  }) => `Promise`<`ApprovalsResponse`\> ; `approve`: (`__namedParameters`: { `expires?`: `Expiration` ; `spender`: `string` ; `tokenId`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `approveAll`: (`__namedParameters`: { `expires?`: `Expiration` ; `operator`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `burn`: (`__namedParameters`: { `tokenId`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `contractInfo`: () => `Promise`<`ContractInfoResponse`\> ; `extension`: (`__namedParameters`: { `msg`: `ICNSNameExecuteMsg`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `isAdmin`: (`__namedParameters`: { `address`: `string`  }) => `Promise`<`IsAdminResponse`\> ; `mint`: (`__namedParameters`: { `extension`: `Metadata` ; `owner`: `string` ; `tokenId`: `string` ; `tokenUri?`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `minter`: () => `Promise`<`MinterResponse`\> ; `nftInfo`: (`__namedParameters`: { `tokenId`: `string`  }) => `Promise`<`NftInfoResponseForMetadata`\> ; `numTokens`: () => `Promise`<`NumTokensResponse`\> ; `ownerOf`: (`__namedParameters`: { `includeExpired?`: `boolean` ; `tokenId`: `string`  }) => `Promise`<`OwnerOfResponse`\> ; `revoke`: (`__namedParameters`: { `spender`: `string` ; `tokenId`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `revokeAll`: (`__namedParameters`: { `operator`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `sendNft`: (`__namedParameters`: { `contract`: `string` ; `msg`: `string` ; `tokenId`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `tokens`: (`__namedParameters`: { `limit?`: `number` ; `owner`: `string` ; `startAfter?`: `string`  }) => `Promise`<`TokensResponse`\> ; `transferNft`: (`__namedParameters`: { `recipient`: `string` ; `tokenId`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `transferrable`: () => `Promise`<`TransferrableResponse`\>  } |
| `IcnsNameNftQueryClient` | { `constructor`: (`client`: `CosmWasmClient`, `contractAddress`: `string`) => [`IcnsNameNftQueryClient`](contracts.md#icnsnamenftqueryclient) ; `client`: `CosmWasmClient` ; `contractAddress`: `string` ; `admin`: () => `Promise`<`AdminResponse`\> ; `allNftInfo`: (`__namedParameters`: { `includeExpired?`: `boolean` ; `tokenId`: `string`  }) => `Promise`<`AllNftInfoResponseForMetadata`\> ; `allOperators`: (`__namedParameters`: { `includeExpired?`: `boolean` ; `limit?`: `number` ; `owner`: `string` ; `startAfter?`: `string`  }) => `Promise`<`OperatorsResponse`\> ; `allTokens`: (`__namedParameters`: { `limit?`: `number` ; `startAfter?`: `string`  }) => `Promise`<`TokensResponse`\> ; `approval`: (`__namedParameters`: { `includeExpired?`: `boolean` ; `spender`: `string` ; `tokenId`: `string`  }) => `Promise`<`ApprovalResponse`\> ; `approvals`: (`__namedParameters`: { `includeExpired?`: `boolean` ; `tokenId`: `string`  }) => `Promise`<`ApprovalsResponse`\> ; `contractInfo`: () => `Promise`<`ContractInfoResponse`\> ; `isAdmin`: (`__namedParameters`: { `address`: `string`  }) => `Promise`<`IsAdminResponse`\> ; `minter`: () => `Promise`<`MinterResponse`\> ; `nftInfo`: (`__namedParameters`: { `tokenId`: `string`  }) => `Promise`<`NftInfoResponseForMetadata`\> ; `numTokens`: () => `Promise`<`NumTokensResponse`\> ; `ownerOf`: (`__namedParameters`: { `includeExpired?`: `boolean` ; `tokenId`: `string`  }) => `Promise`<`OwnerOfResponse`\> ; `tokens`: (`__namedParameters`: { `limit?`: `number` ; `owner`: `string` ; `startAfter?`: `string`  }) => `Promise`<`TokensResponse`\> ; `transferrable`: () => `Promise`<`TransferrableResponse`\>  } |

#### Defined in

[contracts/index.ts:14](https://github.com/interchain-name/icns/blob/4d8bef0/ts/sdk/src/contracts/index.ts#L14)

___

### IcnsRegistrar

• `Const` **IcnsRegistrar**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `IcnsRegistrarClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`IcnsRegistrarClient`](contracts.md#icnsregistrarclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `claim`: (`__namedParameters`: { `name`: `string` ; `referral?`: `string` ; `verifications`: `Verification`[] ; `verifyingMsg`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `fee`: () => `Promise`<`FeeResponse`\> ; `nameByTwitterId`: (`__namedParameters`: { `twitterId`: `string`  }) => `Promise`<`NameByTwitterIdResponse`\> ; `nameNftAddress`: () => `Promise`<`NameNftAddressResponse`\> ; `referralCount`: (`__namedParameters`: { `name`: `string`  }) => `Promise`<`ReferralCountResponse`\> ; `setMintingFee`: (`__namedParameters`: { `mintingFee?`: `Coin`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `setNameNftAddress`: (`__namedParameters`: { `nameNftAddress`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `setVerificationThreshold`: (`__namedParameters`: { `threshold`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `updateVerifierPubkeys`: (`__namedParameters`: { `add`: `string`[] ; `remove`: `string`[]  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `verificationThreshold`: () => `Promise`<`VerificationThresholdResponse`\> ; `verifierPubKeys`: () => `Promise`<`VerifierPubKeysResponse`\> ; `withdrawFunds`: (`__namedParameters`: { `amount`: `Coin`[] ; `toAddress`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\>  } |
| `IcnsRegistrarQueryClient` | { `constructor`: (`client`: `CosmWasmClient`, `contractAddress`: `string`) => [`IcnsRegistrarQueryClient`](contracts.md#icnsregistrarqueryclient) ; `client`: `CosmWasmClient` ; `contractAddress`: `string` ; `fee`: () => `Promise`<`FeeResponse`\> ; `nameByTwitterId`: (`__namedParameters`: { `twitterId`: `string`  }) => `Promise`<`NameByTwitterIdResponse`\> ; `nameNftAddress`: () => `Promise`<`NameNftAddressResponse`\> ; `referralCount`: (`__namedParameters`: { `name`: `string`  }) => `Promise`<`ReferralCountResponse`\> ; `verificationThreshold`: () => `Promise`<`VerificationThresholdResponse`\> ; `verifierPubKeys`: () => `Promise`<`VerifierPubKeysResponse`\>  } |

#### Defined in

[contracts/index.ts:17](https://github.com/interchain-name/icns/blob/4d8bef0/ts/sdk/src/contracts/index.ts#L17)

___

### IcnsResolver

• `Const` **IcnsResolver**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `IcnsResolverClient` | { `constructor`: (`client`: `SigningCosmWasmClient`, `sender`: `string`, `contractAddress`: `string`) => [`IcnsResolverClient`](contracts.md#icnsresolverclient) ; `client`: `SigningCosmWasmClient` ; `contractAddress`: `string` ; `sender`: `string` ; `address`: (`__namedParameters`: { `bech32Prefix`: `string` ; `name`: `string`  }) => `Promise`<`AddressResponse`\> ; `addressByIcns`: (`__namedParameters`: { `icns`: `string`  }) => `Promise`<`AddressByIcnsResponse`\> ; `addresses`: (`__namedParameters`: { `name`: `string`  }) => `Promise`<`AddressesResponse`\> ; `admin`: () => `Promise`<`AdminResponse`\> ; `config`: () => `Promise`<`Config`\> ; `icnsNames`: (`__namedParameters`: { `address`: `string`  }) => `Promise`<`IcnsNamesResponse`\> ; `names`: (`__namedParameters`: { `address`: `string`  }) => `Promise`<`NamesResponse`\> ; `primaryName`: (`__namedParameters`: { `address`: `string`  }) => `Promise`<`PrimaryNameResponse`\> ; `removeRecord`: (`__namedParameters`: { `bech32Address`: `string` ; `name`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `setPrimary`: (`__namedParameters`: { `bech32Address`: `string` ; `name`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\> ; `setRecord`: (`__namedParameters`: { `adr36Info`: `Adr36Info` ; `bech32Prefix`: `string` ; `name`: `string`  }, `fee`: `number` \| `StdFee` \| ``"auto"``, `memo?`: `string`, `funds?`: `Coin`[]) => `Promise`<`ExecuteResult`\>  } |
| `IcnsResolverQueryClient` | { `constructor`: (`client`: `CosmWasmClient`, `contractAddress`: `string`) => [`IcnsResolverQueryClient`](contracts.md#icnsresolverqueryclient) ; `client`: `CosmWasmClient` ; `contractAddress`: `string` ; `address`: (`__namedParameters`: { `bech32Prefix`: `string` ; `name`: `string`  }) => `Promise`<`AddressResponse`\> ; `addressByIcns`: (`__namedParameters`: { `icns`: `string`  }) => `Promise`<`AddressByIcnsResponse`\> ; `addresses`: (`__namedParameters`: { `name`: `string`  }) => `Promise`<`AddressesResponse`\> ; `admin`: () => `Promise`<`AdminResponse`\> ; `config`: () => `Promise`<`Config`\> ; `icnsNames`: (`__namedParameters`: { `address`: `string`  }) => `Promise`<`IcnsNamesResponse`\> ; `names`: (`__namedParameters`: { `address`: `string`  }) => `Promise`<`NamesResponse`\> ; `primaryName`: (`__namedParameters`: { `address`: `string`  }) => `Promise`<`PrimaryNameResponse`\>  } |

#### Defined in

[contracts/index.ts:20](https://github.com/interchain-name/icns/blob/4d8bef0/ts/sdk/src/contracts/index.ts#L20)
