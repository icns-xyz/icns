#!/usr/bin/env bash

set -o pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

CW_PLUS_VERSION=v0.16.0
NETWORK="${1:-local}"
ARGS=${@:2}

deploy_contract() {
    local contract_name="$1"
    local msg="$2"
    local state_file="state.json"

    if [[ $NETWORK = "local" ]]; then
        state_file="state.local.json"
    fi

    cd "$SCRIPT_DIR/../contracts/${contract_name}"
    
    RUSTFLAGS="-C link-arg=-s" cargo wasm
    beaker wasm deploy "$contract_name" --no-rebuild --raw "$msg" --network "$NETWORK" $ARGS 1> /dev/null


    CONTRACT_ADDR=$(cat "$SCRIPT_DIR/../.beaker/$state_file" | jq ".local.\"${contract_name}\".addresses.default" | sed 's/"//g') 
    echo $CONTRACT_ADDR
}

param() {
    cat "$SCRIPT_DIR//params.json" | jq ".$1"
}


echo ">>> Deploying name-nft contract ..."
echo

read -r -d '' MSG <<- EOF
{
    "transferrable": $(param "transferrable"),
    "admins": $(param "admins")
}
EOF
echo "$MSG" | jq

NAME_NFT_CONTRACT_ADDR=$(deploy_contract "icns-name-nft" "$MSG")
echo "NAME_NFT_CONTRACT_ADDR: $NAME_NFT_CONTRACT_ADDR"


echo ">>> Deploying resolver contract ..."
echo

read -r -d '' MSG <<- EOF
{
    "name_address": "$NAME_NFT_CONTRACT_ADDR"
}
EOF
echo "$MSG" | jq

REGISTRAR_CONTRACT_ADDR=$(deploy_contract "icns-resolver" "$MSG")
echo "RESOLVER_CONTRACT_ADDR: $RESOLVER_CONTRACT_ADDR"



echo ">>> Deploying registrar contract ..."
echo

read -r -d '' MSG <<- EOF
{
    "name_nft_addr": "$NAME_NFT_CONTRACT_ADDR",
    "verifier_pubkeys": $(param "verifier_pubkeys"),
    "verification_threshold": $(param "verification_threshold"),
    "fee": $(param "fee")
}
EOF
echo "$MSG" | jq

REGISTRAR_CONTRACT_ADDR=$(deploy_contract "icns-registrar" "$MSG")
echo "REGISTRAR_CONTRACT_ADDR: $REGISTRAR_CONTRACT_ADDR"


echo ">>> Set registar to be name-nft minter ..."
echo

read -r -d '' MSG <<- EOF
{
    "extension": {
        "msg": {
            "set_minter": {
                "minter_address": "$REGISTRAR_CONTRACT_ADDR"
            }
        }
    }
}
EOF
echo "$MSG" | jq

beaker wasm execute icns-name-nft --signer-account test1 --raw "$MSG" 1> /dev/null