# Structural Overview of ICNS

![icns-structure-overview](https://user-images.githubusercontent.com/45252226/206906578-61e36917-c35d-46d5-9f97-42c083f3557a.png)

## Users Registring a new ICNS

As a user registering a new ICNS, the user would be verifying their Twitter handle and then provide the address they would like to use for each bech32 prefix(ex. osmo -> osmo1.....,  juno -> juno1....).

This information would then be verified by different verifiers registered in the Registrar contract. When the verification has reached quorum upon verifiers in the Registrar contract, the Registrar contract would then send a message to the Name-nft contract to claim the ICNS name for the user. We have a modularized contract for the registrar to be able to be open to different mechanics around claiming names, fees, auctions, distribution, open to be developed in conjunction with the community.

Having claimed a ICNS Name, the user is now able to set different addresses for the ICNS name via Resolver.

The resolver is the contract that keeps the state of the list of addresses for each bech32 prefix and ICNS name.

By adopting the concept of Resolver instead of having a merged contract of Resolver + Registry, we are able to consider different implementations and scalability using the advantage of the modularized structure.

Although the default Resolver would persist of the state mentioned above and keep record of user addresses for each coin type, different Resolver contracts does not necessarily have to have the same state entries or the same roles.

A good example of utilizing Resolver for different purposes would be the [uniswap token list](https://tokenlists.org/), where the resolver serves the role of keeping the token list, instead of simply keeping user addresses.

## Wallets integrating ICNS

Wallets using ICNS would be mainly communicating with the Resolver contract in order to resolve between ICNS name and the bech32 address.

For resolving for an ICNS name("alice.osmo" -> "osmo1xxx"), `AddressByIcns` would be a useful query to spotlight.

`PrimaryName`, a query returning the primary name of the reversed resolved address (e.g "osmo1xxx" -> "alice.osmo) would be one of the main queries to use when integrating ICNS with wallets.
