#Define
$CONTRACT: Staking & Farming contract
$TOKEN-A: Farming token
$TOKEN-B: Farming token
$TOKEN-C: Reward token

#Register account
```sh
near call $CONTRACT storage_deposit '{}' --accountId staking-user.testnet
```

#Stake - Transfer token A to staking contract
```sh
near call $TOKEN_A ft_on_transfer ....
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
