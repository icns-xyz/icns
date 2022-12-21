# ICNS API Documentation

Introduction
------------

Welcome to the ICNS API documentation! The ICNS API allows you to interact with the Interchain Nameservice, a decentralized protocol that enables users to register and manage human-readable names on the Interchain.

Contract Address
----------------

*   Name NFT
    *   Mainnet: TBD
*   Resolver
    *   Mainnet: TBD
*   Registrar
    *   Mainnet: TBD

API Endpoints
-------------

### Resolver APIs

*   Query - config
    *   Gets the config of the resolver contract, including the Name-NFT contract address.
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address)/smart/{base64Encoding({ "config": { }})}
    ```
    
*   Query - addresses\_response
    *   Returns a list of (bech32\_prefix, bech32\_address) associated with the given ICNS name
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address)/smart/{base64Encoding({ "addresses": { "name": "{icns_name}" })}
    ```
    
*   Query - address
    *   Gets the address associated with the given an ICNS name
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address}/smart/{base64Encoding({ "icns_names": { "address": "{bech32Address}" }})} 
    ```
    
    *   Note: you can register multiple addresses to a name. check for `name` and `primary_name`
    *   Method to modify the `primary_name` is implemented on the contract, but not on the frontend (future feature)
*   Query - names
    *   Returns a list of names associated with the given address
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address)/smart/{base64Encoding({ "names": { "{bech32Address}" }})}
    ```
    
    *   NOTE: this query only returns name itself, not full icns name
        *   does not include ICNS bech32 suffix
*   Query - icns\_names
    *   Returns a list of all ICNS names and bech32 prefixes associated with the given address
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address)/smart/{base64Encoding({ "icns_names": { "{bech32Address}" }})}
    ```
    
*   Query - primary\_name
    *   Returns the primary ICNS associated with the given address
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address)/smart/{base64Encoding({ "primary_name": { "{bech32Address}" }})}
    ```
    
*   Query - admin
    *   Returns a list of admins queried from the Name NFT contract.
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address)/smart/{base64Encoding({ "admin": { }})}
    ```
    
*   Query - address\_by\_icns
    *   Returns the bech32 addresses for the associated full (including the bech32 prefix) ICNS name.
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address}/smart/{base64Encoding({ "address-by-icns": { "icns": {twitter_username}.{bech32_prefix} }})}
    ```
    
*   Execute Message - set\_record
*   Execute Message - set\_primary
*   Execute Message - remove\_record

### Registrar API

*   Query - verifier\_pub\_keys
    *   Returns public keys of the verifiers
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address}/smart/{base64Encoding({ "verfier_pub_keys": { }})}
    ```
    
*   Query - verification\_threhold
    *   Returns the current threshold percentage required for verification
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address}/smart/{base64Encoding({ "verification_threshold": { }})}
    ```
    
*   Query - name\_nft\_address
    *   Returns the address of the NFT contract
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address}/smart/{base64Encoding({ "name_nft_address": { }})}
    ```
    
*   Query - referral\_count
    *   Returns the number of referrals by the given ICNS name
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address}/smart/{base64Encoding({ "referral_count": { "name": {twitter_username} }})}
    ```
    
*   Query - fee
    *   Returns the current fee required for claiming/minting a new name
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address}/smart/{base64Encoding({ "fee": { } })}
    ```
    
*   Query - name\_by\_twitter\_id
    *   Returns the name of the user by Twitter ID (note that Twitter ID different from Twitter handle) **at the time the name was claimed.**
    
    ```plain
    {endpoint}/cosmwasm/v1/contract/{resolver_contract_address}/smart/{base64Encoding({ "name_by_twitter_id": { "twitter_id": {twitter_id} }})}
    ```
    
*   Execute Message - claim
*   Execute Message - update\_verifier\_pub\_keys
*   Execute Message - set\_verification\_threshold
*   Execute Message - set\_name\_nft\_address
*   Execute Message - set\_minting\_fee
*   Execute Message - withdraw\_funds

### **Name NFT API**

*   Query - admin
    *   Returns the admin address
*   Query - is\_admin
    *   Checks whether the specified address is an admin
*   Query - transferrable
    *   Returns whether Name NFTs are currently transferrable
*   Query - owner\_of
    *   Check the owner of the given token ID (optional filter on including expired owners)
*   Query - approval
    *   Check for the approval response for a given token ID and spender (optional filter on including expired owners)
*   Query - approvals
    *   Check for the approval response for a given token ID (optional filter on including expired owners)
*   Query - all\_operators
*   Query - num\_tokens
*   Query - contract\_info
*   Query - nft\_info
*   Query - all\_nft\_info
*   Query - tokens
*   Query - all\_tokens
*   Query - minter

Examples
--------

**Get a bech32 address from an ICNS name**

```plain
{endpoint}/cosmwasm/v1/contract/{resolver_contract_address}/smart/{base64Encoding({ "address-by-icns": { "icns": {twitter_username}.{bech32_prefix} }})}
```

*   Example: [https://lcd.testnet.osmosis.zone/cosmwasm/wasm/v1/contract/osmo1qjs29tsavxuk39dugs7lny7cc28a57hqa9u9q0ysr2qat96aghrqzgp60x/smart/eyJpY25zX25hbWVzIjogeyJhZGRyZXNzIjogIm9zbW8xdnY2aHJ1cXV6cHR5NHhwa3M5em5rdzhneXM1eDRuc25nNGtlcnkifX0K](https://lcd.testnet.osmosis.zone/cosmwasm/wasm/v1/contract/osmo1qjs29tsavxuk39dugs7lny7cc28a57hqa9u9q0ysr2qat96aghrqzgp60x/smart/eyJpY25zX25hbWVzIjogeyJhZGRyZXNzIjogIm9zbW8xdnY2aHJ1cXV6cHR5NHhwa3M5em5rdzhneXM1eDRuc25nNGtlcnkifX0K)
*   Response

```plain
{
  "data": {"names":["TonyYun9.osmo"],"primary_name":"TonyYun9.osmo"}
}
```

  

**Get a ICNS name of a bech32 address**

```plain
{endpoint}/cosmwasm/v1/contract/{resolver_contract_address}/smart/{base64Encoding({ "address-by-icns": { "icns": {twitter_username}.{bech32_prefix} }})}
```

*   Example: [https://lcd.testnet.osmosis.zone/cosmwasm/wasm/v1/contract/osmo1qjs29tsavxuk39dugs7lny7cc28a57hqa9u9q0ysr2qat96aghrqzgp60x/smart/eyJhZGRyZXNzIjogeyJuYW1lIjogIlRvbnlZdW45IiwgImJlY2gzMl9wcmVmaXgiOiAib3NtbyJ9fQo=](https://lcd.testnet.osmosis.zone/cosmwasm/wasm/v1/contract/osmo1qjs29tsavxuk39dugs7lny7cc28a57hqa9u9q0ysr2qat96aghrqzgp60x/smart/eyJhZGRyZXNzIjogeyJuYW1lIjogIlRvbnlZdW45IiwgImJlY2gzMl9wcmVmaXgiOiAib3NtbyJ9fQo=)
*   Response

```plain
{
  "data": {"address":"osmo1vv6hruquzpty4xpks9znkw8gys5x4nsng4kery"}
}

```

  

For more information on each of these APIs, please refer to GitHub:

*   [Name NFT contract](https://github.com/icns-xyz/icns/blob/main/contracts/icns-name-nft/src/msg.rs)
*   [Registrar contract](https://github.com/icns-xyz/icns/blob/main/contracts/icns-registrar/src/msg.rs)
*   [Resolver contract](https://github.com/icns-xyz/icns/blob/main/contracts/icns-resolver/src/msg.rs)
