# Resolver

The goal of the Resolver contract is keep track of the address for each ICNS name in a stateful manner. It serves the purpose of "resolving" the ICNS Name to the correct address (e.g "alice.osmo" -> osmo1xxx, "alice.juno" -> "juno1xxx"). 

Each user would be given the ability to set bech32 address for each bech32 prefix for all Cosmos SDK-based and IBC-enabled chains that have registered their bech32 prefixes on the SLIP-173 repo (https://github.com/satoshilabs/slips/blob/master/slip-0173.md). Suffixes (i.e. ‘.osmo’) will in most cases match their chains’ bech32 prefixes.

The Resolver contract also supports reverse-resolving, where we map a bech32 address to an ICNS name (e.g osmo1234 -> alice.osmo). This could be used by integrators such as wallets or protocols to reverse resolve the given bech32 address.

Two different ICNS names are allowed to point to a single bech32 address (e.g the case where alice.osmo -> osmo1234, bob,osmo -> osmo1234). However, allowing this brings in a different problem: how do we know which address to reverse resolve given address osmo1234? Should it be alice.osmo or bob.osmo? To solve this problem, the contract stores the state of primary names for each bech32 addresses, allowing storing and querying the primary name set for the bech32 address.

## State

- `Config`: Stores the Name-nft contract address. This is used to verify the ownership of the ICNS name upon different msg execution.
- `Records`: Indexed map of (username, bech32 prefix) -> bech32 address
- `Primary name`: Map of bech32 address to the user name.
- `Signature`: Map of signature bytes to boolean. Stores all the signature used upon setting record. This is stored to prevent replay attacks using duplicate signature.

## Msg

- `SetRecord`

This message is where all the magic happens for the Resolver contract. To prevent users from registering bech32 addresses that they do not have ownership of, the Resolver contract uses ADR36-verification.

We use a struct called `Adr36Info` within the SetRecord message to wrap and pass all ADR36 related information. The struct contains the following field:
- `signer_bech32_address`: bech32 address of the signer of the message.
- `address_hash`: Enum of hashing method used for the given address. It can either be `Cosmos`, which uses Sha256 and Ripemd160 or 'Ethereum', which uses `Keccack256` as the hasing method.
- `pub_key`: pub key bytes of the sender of signer.
- `signature`: bytes of signature from the address
- `signature_salt`: unique signature that has been signed. This salt is used to prevent double signing for the signature.

The data for ADR36 that user is providing signature for should be in the following format:

```
The following is the information for ICNS registration for alice.osmo.
    
Chain id: <CHAIN_ID>
Contract Address: <RESOLVER_CONTRACT_ADDR>
Owner: juno1....
Salt: <UNIQUE_SIGNATURE_SALT>
```

Note that Owner does not have to match the ICNS name that the user is setting record for.

The ADR36 verification would not take place when the signer of the message and the pub key in `ADR36Info` matches. But instead, to verify that this was intentional, the message requires the signature field and the signature_salt field to be empty.

The most recent address that has been set would be automatically set as the primary name that is to be reversed resolved.


- `SetPrimary`: Allows user to change primary name that is to be reversed resolved for each name -> bech32 address pair. Only single address could be set as the primary name. 

- `RemoveRecord`: Allows user to remove an address that has been mapped to ICNS name + bech32 prefix pair (e.g remove osmo1xxxx for alice.osmo). Note that the contract does not allow removing record / address when there are multiple addresses existing for the ICNS name + bech32 prefix pair. This is to prevent having a record without a primary address to reverse resolve. The only case an account is allowed to have no primary address for ICNS name + bech32 prefix pair is when there is no address for the pair.

## Query
- `Config`: returns the configuration of the Resolver contract which contains the Name-nft contract address.
- `Addresses`: returns list of tuple consisted of (bech32_prefix, bech32_address) for the given ICNS name.

    - e.g) Given "alice" returns [("osmo", "osmo1xxx"), ("juno", "juno1xxx")])
    
- `Address`: returns the bech32 address set for the given name and bech32 prefix. Returns error when address does not exist.
    - e.g) Given "alice" and "osmo" returns "osmo1xxx"

- `Names`: returns names bound to an address and the primary name of given bech32 address. Only returns name itself, not full icns name. (e.g "alice", "bob")
    - e.g) Given "osmo1xxxx" returns ["alice", "bob"] and primary name: alice

- `IcnsNames`: returns list of full icns name given bech32 address, along with primary ICNS name 
    - e.g) given "osmo1xxxx" returns ["alice.osmo", "bob.osmo"], primary name: alice.osmo
- `PrimaryName`: returns the primary name of the address
    - e.g) given "osmo1xxxx" returns "alice.osmo"
- `Admin`: returns the admin of the Name-nft contract
- `AddressByIcns`: returns bech32 addresses for the given full ICNS name.
    - e.g) given "alice.osmo" returns "osmo1xxxx"