# ICNS

## Structural Overview

ICNS is designed with three different contracts: The Resolver, Registry and the Registrar. These concepts are adopted from the ENS Terminology glossary (https://docs.ens.domains/terminology) although the actual implementation differs from how ENS has been implemented. This document would be addressing 
- The user stories and the entry points of ICNS as a whole
- The roles of `Resolver`, `Registry`, and `Registrar`
- The wiring between the `Resolver`, `Registry` and `Registrar`


## User stories and Entry Points

### User Registring a new ICNS

As a user registering a new ICNS, the user would be verifying their Twitter handle and then provide the address they would like to use for each bech32 prefix(ex. osmo -> osmo1.....,  juno -> juno1....). 

This information would then be verified by different operators of the Registrar contract. When the verification has passed the passing threshold within the Registrar contract, the Registrar contract would then send a message to the Registry contract and the Resolver contract respectively.

The Registry contract mints a NFT of the ICNS to the user address.

The Resolver would then save the address for each bech32 prefix.


### User Querying an address for a specific coin type

A user wanting to query an address for a specific coin type for a specified user name would send a query to the Registry contract. The Registry contract would send a query to the resolver contract to query the address for the given user name + coin type and return the results to the user. 


## Resolver
The resolver is the contract that keeps the state of the list of addresses for each bech32 prefix.

### State
We achieve this goal by keeping a map of (user name, bech32 prefix) -> Adress(String) within the contract.

Each (user name + bech32 prefix) combination would have a distinct address. For example, for the user with the user name of "Bob" the address stored for prefix "cosmos" would be "cosmos1...", whilst the address for prefix of "osmo" would be "osmo1....". Bob wants to be able to connect multiple addresses to the ICNS. This could be done by keeping the state entry of address for each bech32 prefix per user.

### Config and Instatiation

As a part of instantiation, the contract requires an the address of the Registrar contract.

Only the registrar contract is able to call the Resolver and set addresses for the user. 

### Scalability of Resolver

By adopting the concept of Resolver instead of having a merged contract of Resolver + Registry, we are able to consider different implementations and scalability using the advantage of the modularized structure.

Although the default Resolver would persist of the state mentioned above and keep record of user addresses for each coin type, different Resolver contracts does not necessarily have to have the same state entries or the same roles.

A good example of utilizing Resolver for different purposes would be the uniswap token list (https://tokenlists.org/), where the resolver serves the role of keeping the token list, instead of simply keeping user addresses.

## Registrar

The Registrar is the contract that would be the main entry point for the users in registrering a new record for ICNS. 

### Config and Instatiation

As a part of instantiation, the contract requires an address(es) of the operators(which would most likely be the backend in charge of verifying Twitter OAuth in the beginning), and the addresses of the Resolver contract and the Registry contract.

Note that the Admin of the Registry contract has the ability to add or remove existing operators. 

Upon verification, or passing the verification threshold, the Registrar contract would be sending a message to the Registry contract and the Resolver contract respectively to mint NFT for the given user name and save the addresses for each bech32 prefixes.


### State

The Registrar contract keeps a state of message counts for each user name. 

Each time an operator(most likely a backend with a private key) address verifies the user name, they would be sending a message to the Registrar as a proof that they have verified, along with the given user input of address for each bech32 prefix.
We keep a state entry of how many admins have sent this verifying message for each user name, and if the verifying message has exceeded the passing threshold (50% of the registered admin), the Registrar would then send a message to the Registry contract. 

## Registry
The Registry is the contract in charge of minting the NFT for the given user name and manage the ownership of the NFT. It also serves the purpose of orchestration between each user name and different Resolvers.

### State
The Registry does this by keeping a map of user name -> Different Resolver contract addresses.

Although the default Resolver contract would be the address resolver, we can also think about scalability here by being able to set different resolvers for each user name. (ex. token list, nft resolver etc...)

### Config and Instatiation

As a part of instantiation, the contract requires an address of an admin, and the address of the Registrar contract for the `Config`. 

Only the registered Registrar contract and the admin is allowed to send messages to the Registry and mint the NFT for the given user name.

An admin is allowed to add or remove a different admin, or add or remove an existing Registrar address. This admin is also used across the default Registrar contract and the Resolver contract. The admin of Registry is able to add or remove operators from the Registrar contract and could also over write / set addresses in the Resolver contract. 
