WALLET_PEM="/Users/sorinpetreasca/DevKitt/walletKey.pem"
WALLET_PEM2="/Users/sorinpetreasca/DevKitt/walletKey2.pem"
WALLET_PEM3="/Users/sorinpetreasca/DevKitt/walletKey3.pem"
PROXY="https://testnet-gateway.dharitri.com"
CHAIN_ID="T"

LIQUID_STAKING_WASM_PATH="/Users/sorinpetreasca/Dharitri/sc-liquid-staking-rs/liquid-staking/output/liquid-staking.wasm"

OWNER_ADDRESS="moa14nw9pukqyqu75gj0shm8upsegjft8l0awjefp877phfx74775dsqcakpap"
CONTRACT_ADDRESS="moa1qqqqqqqqqqqqqpgq4dzfldya86ht366xrd4w78809rlxcfzn5dsqqt0x7w"

deploySC() {
    moapy --verbose contract deploy --recall-nonce \
        --bytecode=${LIQUID_STAKING_WASM_PATH} \
        --pem=${WALLET_PEM} \
        --gas-limit=100000000 \
        --metadata-payable-by-sc \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

upgradeSC() {
    moapy --verbose contract upgrade ${CONTRACT_ADDRESS} --recall-nonce \
        --bytecode=${LIQUID_STAKING_WASM_PATH} \
        --pem=${WALLET_PEM} \
        --gas-limit=100000000 \
        --metadata-payable-by-sc \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

TOKEN_NAME=0x4c53544f4b454e #LSTOKEN
TOKEN_TICKER=0x4c5354 #LST
TOKEN_DECIMALS=18
registerLsToken() {
    moapy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=100000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --value=50000000000000000 \
        --function="registerLsToken" \
        --arguments ${TOKEN_NAME} ${TOKEN_TICKER} ${TOKEN_DECIMALS} \
        --send || return
}

UNSTAKE_TOKEN_NAME=0x4c53554e5354414b45 #LSUNSTAKE
UNSTAKE_TOKEN_TICKER=0x4c5355 #LSU
registerUnstakeToken() {
    moapy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=100000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --value=50000000000000000 \
        --function="registerUnstakeToken" \
        --arguments ${UNSTAKE_TOKEN_NAME} ${UNSTAKE_TOKEN_TICKER} ${TOKEN_DECIMALS} \
        --send || return
}

setStateActive() {
    moapy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="setStateActive" \
        --send || return
}

setStateInactive() {
    moapy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="setStateInactive" \
        --send || return
}

###PARAMS 
### Contracts - moa1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqq80llllswp8ex4 moa1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqxlllllskp38gp
DELEGATION_ADDRESS="moa1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqq80llllswp8ex4"
TOTAL_STAKED=1000000000000000000000
DELEGATION_CAP=3000000000000000000000
NR_NODES=3
APY=1974
whitelistDelegationContract() {
    moapy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=10000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="whitelistDelegationContract" \
        --arguments ${DELEGATION_ADDRESS} ${OWNER_ADDRESS} ${TOTAL_STAKED} ${DELEGATION_CAP} ${NR_NODES} ${APY}\
        --send || return
}

NEW_TOTAL_STAKED=1500000000000000000000
NEW_DELEGATION_CAP=5000000000000000000000
NEW_APY=18830
changeDelegationContractParams() {
    moapy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="changeDelegationContractParams" \
        --arguments ${DELEGATION_ADDRESS} ${NEW_TOTAL_STAKED} ${NEW_DELEGATION_CAP} ${NR_NODES} ${NEW_APY}\
        --send || return
}

###PARAMS
#1 - Amount
addLiquidity() {
        moapy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --value=$1 \
        --function="addLiquidity" \
        --send || return
}

###PARAMS
#1 - Amount
LS_TOKEN=str:LST-05ef8b
REMOVE_LIQUIDITY_METHOD=str:removeLiquidity
removeLiquidity() {
    moapy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="DCTTransfer" \
        --arguments ${LS_TOKEN} $1 ${REMOVE_LIQUIDITY_METHOD} \
        --send || return
}

###PARAMS
## TESTING: 1437 unstake epoch
#1 - Nonce
unbondTokens() {
    user_address="$(moapy wallet pem-address $WALLET_PEM2)"
    method_name=str:unbondTokens
    unbond_token=str:LSU-1cd7b4
    unbond_token_nonce=$1
    UNBOND_TOKEN_AMOUNT=1
    moapy --verbose contract call $user_address --recall-nonce \
        --pem=${WALLET_PEM2} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="DCTNFTTransfer" \
        --arguments $unbond_token $1 ${UNBOND_TOKEN_AMOUNT} ${CONTRACT_ADDRESS} $method_name \
        --send || return
}

claimRewards() {
    moapy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="claimRewards" \
        --send || return
}

recomputeTokenReserve() {
    moapy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="recomputeTokenReserve" \
        --send || return
}

delegateRewards() {
    moapy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --function="delegateRewards" \
        --send || return
}

# VIEWS

getLsTokenId() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getLsTokenId" \
}

###PARAMS
#1 - Amount
getLsValueForPosition() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --arguments $1 \
        --function="getLsValueForPosition" \    
}

getUnstakeTokenId() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getUnstakeTokenId" \
}

getUnstakeTokenSupply() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getUnstakeTokenSupply" \
}

getVirtualMoaxReserve() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getVirtualMoaxReserve" \
}

getRewardsReserve() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getRewardsReserve" \
}

getTotalWithdrawnMoax() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getTotalWithdrawnMoax" \
}

getDelegationAddressesList() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getDelegationAddressesList" \
}

###PARAMS
#1 - Address
getDelegationContractData() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --arguments $1 \
        --function="getDelegationContractData" \
}

getDelegationStatus() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getDelegationStatus" \
}

getDelegationClaimStatus() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --function="getDelegationClaimStatus" \
}

getDelegationContractStakedAmount() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --arguments ${DELEGATION_ADDRESS} \
        --function="getDelegationContractStakedAmount" \
}

getDelegationContractUnstakedAmount() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --arguments ${DELEGATION_ADDRESS} \
        --function="getDelegationContractUnstakedAmount" \
}

getDelegationContractUnbondedAmount() {
    moapy --verbose contract query ${CONTRACT_ADDRESS} \
        --proxy=${PROXY} \
        --arguments ${DELEGATION_ADDRESS} \
        --function="getDelegationContractUnbondedAmount" \
}
