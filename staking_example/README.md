#Define
$CONTRACT: Staking & Farming contract
$TOKEN-A: Farming token
$TOKEN-B: Farming token
$TOKEN-C: Reward token

#Register account
```sh
near call $CONTRACT_NAME storage_deposit '{}' --accountId user1-stakenet.testnet --deposit 1
```

# (Optional) Transfer token A & B to "user1-stakenet.testnet"
```sh
export TOKEN_A=dev-1652170997032-43490833923360
near call $TOKEN_A storage_deposit '' --accountId user1-stakenet.testnet --amount 0.00125
near call $TOKEN_A ft_transfer '{"receiver_id": "user1-stakenet.testnet", "amount": "100000000000000000"}' --accountId $TOKEN_A --amount 0.000000000000000000000001

export TOKEN_B=dev-1652171111079-16054088636054
near call $TOKEN_B storage_deposit '' --accountId user1-stakenet.testnet --amount 0.00125
near call $TOKEN_B ft_transfer '{"receiver_id": "user1-stakenet.testnet", "amount": "100000000000000000"}' --accountId $TOKEN_B --amount 0.000000000000000000000001

near call $TOKEN_A storage_deposit '' --accountId $CONTRACT_NAME --amount 0.00125
near call $TOKEN_B storage_deposit '' --accountId dev-1652116452491-16143624595494 --amount 0.00125
```
#Stake - Transfer token A to staking contract
```sh
near call $TOKEN_A ft_transfer_call '{"receiver_id": "'$CONTRACT_NAME'", "amount": "100", "msg": ""}' --accountId user1-stakenet.testnet --depositYocto 1 --gas 300000000000000

#check pool balance
near view $CONTRACT_NAME get_staking_pool_info '{"contract_id": "'$TOKEN_A'"}' 
```

#Unstake - Unstake feature 
```sh
near call $CONTRACT unstake '{"amount": 100}' --accountId staking-user.testnet

``` 

#Withdraw - Only allow withdraw in the next epoch
```sh
near call $CONTRACT withdraw '{}' --accountId staking-user.testnet
```

#Harvest - claim reward - Token C
```sh
near call $CONTRACT harvest '{}' --accountId staking-user.testnet
```

#Transfer token B to staking contract
