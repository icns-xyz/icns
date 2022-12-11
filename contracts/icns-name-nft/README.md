# Name-nft Contract

The responsibility of the icns-name-nft contract is to mint the nft of the ICNS name being claimed. The Registrar contract is set as the minter by default, and only the Registrar contract would have the ability to call the minting msg.

This contract extends the CW-721 contract for nfts, and has extra functionality to mange list of admins being used cross-contract, to manage the transferability of the Nft. Admin would have the ability to change the transferability of the Nft minted by the Name-nft contract. 

The token-id of the Nft being minted would be directly set as the user name being claimed, whilst the referrer would be set in the Metadata of the Nft being minted if it exists.

The roles of admin from the Name nft contract is as the following:


1. Registrar Contract
    - Claim name without having verifier's signatures and secp256k1 verifiaction
    - Change verification threshold
    - Update the list of verifiers in charge of watching the Twitter API
    - Change the Name-nft contract Registrar contract is pointed at
    - Set and change minting fees for each ICNS name.
    - Withdraw collected fees from minting fee.

2. Name Nft Contract
    - Change transferability of ICNS Name Nfts.
    - Ability to transfer Name nfts. 
    - Set minter address for the Nfts. This is set to registrar by default.

3. Resolver Contract
    - Skip ADR-36 verification upon setting record for address.
    - Set and change primary name for name and address.
    - Remove a record for ICNS name + bech32 address pair.

