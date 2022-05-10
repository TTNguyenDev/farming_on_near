use std::collections::HashMap;

use crate::*;

#[near_bindgen]
impl StakingContract {
    pub(crate) fn internal_unstake(
        &mut self,
        account_id: AccountId,
        amount: u128,
        contract_id: AccountId,
    ) {
        let mut account = self.accounts.get(&account_id).unwrap();

        assert!(
            amount
                <= *account
                    .stake_balance
                    .get(&contract_id.clone())
                    .unwrap_or(&0),
            "ERR_AMOUNT_MUST_LESS_THAN_STAKE_BALANCE"
        );

        let new_reward = self.internal_calculate_account_reward(&account, contract_id.clone());

        // update account data
        account.add_reward(&contract_id.clone(), new_reward);
        account.sub_stake(&contract_id.clone(), amount);

        self.accounts.insert(&account_id, &account);

        //NOTE: Update global pool
        let mut staking_pool = self
            .staking_pools
            .get(&contract_id.clone())
            .expect("Pool not found");
        let new_contract_reward = self.internal_calculate_global_reward(contract_id.clone());
        staking_pool.pre_reward += new_contract_reward;
        staking_pool.last_block_balance_change = env::block_index();
        staking_pool.total_stake_balance -= amount;
        self.staking_pools
            .insert(&contract_id.clone(), &staking_pool);
    }

    pub(crate) fn internal_withdraw(&mut self, account_id: AccountId) -> Account {
        let account = self.accounts.get(&account_id).unwrap();

        // assert!(
        //     account.unstake_balance.get(&contract_id).unwrap(&0) > 0,
        //     "ERR_UNSTAKE_BALANCE_EQUAL_ZERO"
        // );
        // assert!(
        //     account.unstake_available_epoch <= env::epoch_height(),
        //     "ERR_DISABLED_WITHDRAW"
        // );

        // let new_account = Account {
        //     stake_balance: account.stake_balance,
        //     pre_reward: account.pre_reward,
        //     last_block_balance_change: account.last_block_balance_change,
        //     unstake_balance: 0,
        //     unstake_start_timestamp: 0,
        //     unstake_available_epoch: 0,
        //     new_account_data: account.new_account_data,
        // };

        // self.accounts.insert(&account_id, &new_account);

        account
    }

    pub(crate) fn internal_deposit_and_stake(&mut self, account_id: AccountId, amount: u128) {
        // Validate data
        env::log(format!("Stake amount: {}", amount).as_bytes());
        let account = self.accounts.get(&account_id);
        assert!(account.is_some(), "ERR_ACCOUNT_NOT_FOUND");

        //TODO: Check contract of token A || token B
        let mut staking_pool = self
            .staking_pools
            .get(&env::predecessor_account_id())
            .expect("ERR_INVALID_FT_CONTRACT_ID");

        let mut account = Account::from(account.unwrap());
        let new_reward =
            self.internal_calculate_account_reward(&account, env::predecessor_account_id());

        // update account data
        account.add_reward(&account_id.clone(), new_reward);
        account.add_stake(&account_id.clone(), amount);
        self.accounts.insert(&account_id, &account);

        // Update pool data
        let new_contract_reward =
            self.internal_calculate_global_reward(env::predecessor_account_id());
        staking_pool.total_stake_balance += amount;
        staking_pool.pre_reward += new_contract_reward;
        staking_pool.last_block_balance_change = env::block_index();
        self.staking_pools
            .insert(&env::predecessor_account_id(), &staking_pool);
    }

    pub(crate) fn internal_register_account(&mut self, account_id: AccountId) {
        let account = Account {
            stake_balance: HashMap::new(),
            pre_reward: HashMap::new(),
            last_block_balance_change: HashMap::new(),
        };

        self.accounts.insert(&account_id, &account);
    }

    pub(crate) fn internal_calculate_account_reward(
        &self,
        account: &Account,
        contract_id: AccountId,
    ) -> Balance {
        let lasted_block = env::block_index();

        let diff_block = lasted_block
            - account
                .last_block_balance_change
                .get(&contract_id)
                .unwrap_or(&0);
        let pool_info = self
            .staking_pools
            .get(&contract_id)
            .expect("Pool not found");

        if pool_info.total_stake_balance == 0 {
            return 0;
        }

        let reward_per_block = REWARD_PER_BLOCK * pool_info.weight as u128;
        // NOTE: stake_balance * reward_per_block * diff_block
        let stake_balance: Balance = account.get_stake_balance(contract_id);
        (stake_balance * reward_per_block as u128 * diff_block as u128)
            / pool_info.total_stake_balance as u128
    }

    //TODO: Get deployed block_index of reward token
    pub(crate) fn internal_calculate_global_reward(&self, contract_id: AccountId) -> Balance {
        let lasted_block = env::block_index();

        let pool_info = self
            .staking_pools
            .get(&contract_id)
            .expect("Pool not found");
        let diff_block = lasted_block - pool_info.last_block_balance_change;
        // //NOTE: total_stake_balance * reward_per_block * diff_block

        let reward_per_block = REWARD_PER_BLOCK * pool_info.weight as u128;
        let reward: Balance = diff_block as u128 * reward_per_block;

        reward
    }
}
