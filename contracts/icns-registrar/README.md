# icns-registrar

The main responsibility of the Registrar contract is to work as a contract to orchestrate the oracle verifiers watching the Twitter API for unique Twitter handles of the user. The goal of the Registrar contract is to prevent a single centeralized entity being able to calim a user name for ICNS. This is achieved by keeping a constant for the quorum to be met upon verifiers, and the name being able to be claimed **only if** the quorum has been met for a single name.

Upon quorum being met, the Registrar contract calls the Name-nft contract which then mints Nft for the corresponding name being claimed.

## State

The State kept within the contract are the following:
1. Config

    - name_nft: the contract address of the Name-nft contract. This is used to send msg for creating a Nft upon upon claiming an ICNS name.
    - verifier_pubkeys: list of sec1 encoded pubkey bytes of verifier. 
    - verification_threshold_percentage: quorum that has to be met upon verifiers for the name to be claimed. 
    - fee: fee required for the name to be claimed.
2. Referral

    keeps count of the referral count for each user name

3. Unique Twitter ID

    Keeps a map of unique twitter id for each claimed name. This state is stored to prevent users from claiming multiple ICNS by simply changing they're Twitter handle by using the unique twitter id each Twitter account has.

## Msg

- `Claim`: A client sending this msg is expected to collect the signatures from each verifiers. By using secp256k1 on the collected signatures for each verifiers, the contract counts how many of the verifiers has agreed, proved and verified upon the ownership of the corresponding user. If quorum has been met, the contract calls Name-nft contract to mint the Nft of the name being claimed. 

- `SetVerificationThreshold`: changes the quorum needed to be met upon verifiers to claim a name. Only admin can change the verification threshold. 

- `UpdateVerifierPubkeys`: updates the list of verifiers, in charge of watching the Twitter API as an oracle. Only admin can change the verification threshold. 

- `SetNameNftAddress`: sets the Name-nft contract address. The contract set using this msg would be called to mint the Nft for the ICNS name. Only admin can set the Name nft address.

- `SetMintingFee`: changes the minting fee required to mint an icns name.

- `WithdrawFunds`: Withdraws the fees collected via minting fee.


## Query
- `VerifierPubKeys`: returns all the public keys of the verifiers
- `VerificationThreshold`: returns the threshold percentage of verification signature required out of all verifiers
- `NameNftAddress`: returns the address of the name NFT contract
- `ReferralCount`: returns the number of referral for a specific user(name)
- `Fee`: returns the current fee required for minting new name
- `NameByTwitterId`: returns the name of the user by twitter id. Note that the name returned does not indicate the "current" name of the user in Twitter, but the name that the user has used when claiming icns.