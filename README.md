# ICNS

## Structural Overview

ICNS is designed with three different contracts: The Resolver, Registry and the Registrar. These concepts are adopted from the ENS Terminology glossary (https://docs.ens.domains/terminology) although the actual implementation differs from how ENS has been implemented. This document would be addressing 
- The user stories and the entry points of ICNS as a whole
- The roles of `Resolver`, `Registry`, and `Registrar`
- The wiring between the `Resolver`, `Registry` and `Registrar`


## User stories and Entry Points

### User Registring a new ICNS

As a user registering a new ICNS, the user would be verifying their Twitter handle and then provide the address they would like to use for each coin type. 

This information would then be verified by different admins of the Registrar contract. When the verification has passed the passing threshold within the Registrar contract, the Registrar contract would then send a message to the Registry contract. 

The Registry contract saves the state of the owner for the given user name, and passes the information of address per each coin type to the Resolver contract. 

Finally, the Registry contract saves the state of the user name.


### User Querying an address for a specific coin type

A user wanting to query an address for a specific coin type for a specified user name would send a query to the Registry contract. The Registry contract would send a query to the resolver contract to query the address for the given user name + coin type and return the results to the user. 

<img width="730" alt="Screen Shot 2022-11-15 at 5 36 21 PM" src="https://user-images.githubusercontent.com/45252226/201870268-764d9c46-54e5-4f03-bdbb-fceb5bd41bb5.png">


## Resolver
The resolver is the contract that keeps the state of the list of addresses.

### State
We achieve this goal by keeping a map of (user name, coin type) -> Adress(Bytes) within the contract.

Each (user name + coin type) combination would have a distinct address. For example, for the user with the user name of "Bob" the address for coin type 60 would be "0x1234...", whilst the address for coin type 118 would be "cosmos1....". Bob wants to be able to connect multiple addresses to the ICNS. This could be done by keeping the state entry of address for each coin type per user.

### Config and Instatiation

As a part of instantiation, the contract requires an address of an admin, and the address of the Registry contract. 

The resolver contract only allows setting a new user address or altering the existing user address if the sender of the message is the Registry contract or the Admin.

### Scalability of Resolver

By adopting the concept of Resolver instead of having a merged contract of Resolver + Registry, we are able to consider different implementations and scalability using the advantage of the modularized structure.

Although the default Resolver would persist of the state mentioned above and keep record of user addresses for each coin type, different Resolver contracts does not necessarily have to have the same state entries or the same roles.

A good example of utilizing Resolver for different purposes would be the uniswap token list (https://tokenlists.org/), where the resolver serves the role of keeping the token list, instead of simply keeping user addresses. 

## Registrar

The Registrar is the contract that would be the main entry point for the users in registrering a new record for ICNS. 

### Config and Instatiation

The Config would be containing the addresses of admins. Only the registered admins are allowed to send a transaction to the Registrar contract. 

The Config would also be containing an address of the Registry contract, of which it would be sending messages to to set and save the actual record of the user.

### State

The Registrar contract keeps a state of message counts for each user name. 

Each time an admin address verifies the user name, owner and the addresses per coin typem they would be sending a message to the Registrar as a proof that they have verified. We keep a state entry of how many admins have sent this verifying message for each user name, and if the verifying message has exceeded the passing threshold (50% of the registered admin), the Registrar would then send a message to the Registry contract. 


## Registry
The Registry is the main contract that handles the orchestration between the Resolver and the Registrar. 

### State
The Registry does this by keeping two different states within the contract. 

1. A map of user name -> owner

The registry keeps the state of the owner for each user name.

2. A map of user name -> Resolver contract address

The registry keeps the state of the resolver contract address for each user name. 

### Config and Instatiation

As a part of instantiation, the contract requires an address of an admin, and the address of the Registrar contract for the `Config`.  Only the registered Registrar contract and the admin is allowed to send messages to the Registry and save a new user record.

An admin is allowed to add or remove a different admin, or add or remove an existing Registrar address.
