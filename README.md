# ICNS

## A Multichain Native Nameservice
ICNS is a nameservice designed from the ground up with the unique needs of a multichain IBC ecosystem in mind. While every Cosmos chain is distinct, it is simply terrible UX for them to have independent, fragmented name services. Indeed, just as unique addresses are not required to interact with different apps on Ethereum, neither should they be required to interact with different chains in Cosmos. Interchain Nameservice (ICNS) aims to make this a reality by providing a fair, readily adoptable naming protocol for any chain connected with IBC.

With ICNS, users will be able to own a single name that represents their identity across the entire Cosmos ecosystem, while also differentiating between their accounts on different chains. Much like bech32 prefixes identify an address’ corresponding chain (e.g. osmo1 for Osmosis, cosmos1 for Cosmos Hub,), ICNS names attach to suffixes that represent different chain-level domains, thereby allowing one name to specify resolution addresses in many name spaces. For example, a user that owns the @alice ICNS name, will be able to set their alice.osmo resolution for Osmosis, alice.cosmos for Cosmos Hub, and alice.juno for Juno.

## Structural Overview

ICNS is designed with three different contracts: The Resolver, Registrar and the Name-nft contract. These concepts are adopted from the ENS Terminology glossary (https://docs.ens.domains/terminology) although the actual implementation differs from how ENS has been implemented. This roles of each contract is as the following:
- Registrar contract: Orchestrates the oracale verifiers watching the Twitter API for unique Twitter handles of the user. Handles signature verifications and passing threshold of the oracle verifiers.
- Name nft contract: Extends CW-721 contract, mints NFT of the name being claimed.
- Resolver contract: Keeps state of records for each user name(e.g alice.osmo -> alice1....). Also keeps state for revered resolver(e.g alice1.... -> alice.osmo)

## Documentation
Further documentations for each contract can be found in each repo's README:
- Registrar: https://github.com/icns-xyz/icns/tree/main/contracts/icns-registrar
- Resolver: https://github.com/icns-xyz/icns/tree/main/contracts/icns-resolver
- Name-Nft: https://github.com/icns-xyz/icns/tree/main/contracts/icns-name-nft

For further information about the structural overview or the details needed for further integration of ICNS, please visit https://github.com/icns-xyz/icns/tree/main/docs
