# Define
### $CONTRACT_NAME: Staking & Farming contract
### $TOKEN-A: Farming token
### $TOKEN-B: Farming token
### $TOKEN-C: Reward token

#Register account
```sh
near call $CONTRACT_NAME new '{"owner_id": "unify.testnet"}' --accountId $CONTRACT_NAME
near call $CONTRACT_NAME storage_deposit '{}' --accountId user1-stakenet.testnet --deposit 1
```

# (Optional) Transfer token A & B to "user1-stakenet.testnet"
```sh
export TOKEN_A=dev-1652170997032-43490833923360
export TOKEN_B=dev-1652171111079-16054088636054
export TOKEN_C=dev-1652170737474-84884478123817

near call $TOKEN_A storage_deposit '' --accountId user1-stakenet.testnet --amount 0.00125
near call $TOKEN_A ft_transfer '{"receiver_id": "user1-stakenet.testnet", "amount": "100000000000000000"}' --accountId $TOKEN_A --amount 0.000000000000000000000001

near call $TOKEN_B storage_deposit '' --accountId user1-stakenet.testnet --amount 0.00125
near call $TOKEN_B ft_transfer '{"receiver_id": "user1-stakenet.testnet", "amount": "100000000000000000"}' --accountId $TOKEN_B --amount 0.000000000000000000000001

near call $TOKEN_A storage_deposit '' --accountId $CONTRACT_NAME --amount 0.00125
near call $TOKEN_B storage_deposit '' --accountId $CONTRACT_NAME --amount 0.00125
near call $TOKEN_C storage_deposit '' --accountId $CONTRACT_NAME --amount 0.00125
```
# Stake - Transfer token A to staking contract
```sh
near call $TOKEN_A ft_transfer_call '{"receiver_id": "'$CONTRACT_NAME'", "amount": "100", "msg": ""}' --accountId user1-stakenet.testnet --depositYocto 1 --gas 300000000000000

#check pool balance
near view $CONTRACT_NAME get_staking_pool_info '{"contract_id": "'$TOKEN_A'"}' 

near view $CONTRACT_NAME get_staking_pool_info '{"contract_id": "'$TOKEN_B'"}' 
```

# Unstake - Unstake and automatically send token to your near wallet
```sh
near call $CONTRACT_NAME unstake '{"contract_id": "'$TOKEN_A'"}' --accountId user1-stakenet.testnet --depositYocto 1 --gas 300000000000000

``` 

# Harvest - claim reward - Token C
```sh
#View reward amount 

#Add Reward token to wallet - storage_deposit
near call $TOKEN_C storage_deposit '' --accountId user1-stakenet.testnet --amount 0.00125

near call $CONTRACT_NAME harvest '{"contract_id": "'$TOKEN_A'"}' --accountId user1-stakenet.testnet --depositYocto 1 --gas 300000000000000
```

# Transfer token B to staking contract
