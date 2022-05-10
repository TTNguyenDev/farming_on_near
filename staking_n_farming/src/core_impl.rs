use crate::constants::*;
use crate::*;

#[ext_contract(ext_reward_contract)]
pub trait RewardToken {
    fn mint(&mut self, to: AccountId, amount: Balance);
}

pub trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[ext_contract(ext_ft_contract)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(ext_self)]
pub trait ExtStakingContract {
    fn ft_transfer_callback(&mut self, amount: U128, account_id: AccountId, contract_id: AccountId);
    fn ft_withdraw_callback(
        &mut self,
        account_id: AccountId,
        contract_id: AccountId,
        balance: Balance,
    ) -> U128;
    fn mint_reward_token_callback(
        &mut self,
        account_id: AccountId,
        contract_id: AccountId,
        balance: Balance,
    );
}

#[near_bindgen]
impl FungibleTokenReceiver for StakingContract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        self.internal_deposit_and_stake(sender_id, amount.0);

        PromiseOrValue::Value(U128(0))
    }
}

#[near_bindgen]
impl StakingContract {
    //NOTE: For this demo, Only allow unstake all
    #[payable]
    pub fn unstake(&mut self, contract_id: AccountId) -> Promise {
        assert_one_yocto();

        let account_id = env::predecessor_account_id();
        let amount = self.internal_unstake(account_id.clone(), contract_id.clone());

        //NOTE: Withdraw immediately
        ext_ft_contract::ft_transfer(
            account_id.clone(),
            U128(amount),
            Some("Staking contract withdraw".to_string()),
            &contract_id,
            DEPOSIT_ONE_YOCTO,
            FT_TRANSFER_GAS,
        )
        .then(ext_self::ft_withdraw_callback(
            account_id.clone(),
            contract_id,
            amount,
            &env::current_account_id(),
            NO_DEPOSIT,
            FT_HARVEST_CALLBACK_GAS,
        ))
    }

    #[payable]
    pub fn harvest(&mut self, contract_id: AccountId) -> Promise {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let account = self.accounts.get(&account_id).unwrap();

        let new_reward: Balance =
            self.internal_calculate_account_reward(&account, contract_id.clone());
        let current_reward: Balance =
            account.pre_reward.get(&contract_id).unwrap_or(&0) + new_reward;
        assert!(current_reward > 0, "ERR_REWARD_EQUAL_ZERO");

        ext_reward_contract::mint(
            account_id.clone(),
            current_reward,
            &REWARD_TOKEN_ACCOUNT_ID.to_string(),
            NO_DEPOSIT,
            FT_HARVEST_CALLBACK_GAS,
        )
        .then(ext_self::mint_reward_token_callback(
            account_id.clone(),
            contract_id.clone(),
            current_reward,
            &env::current_account_id(),
            NO_DEPOSIT,
            FT_HARVEST_CALLBACK_GAS,
        ))
    }

    #[private]
    pub fn mint_reward_token_callback(
        &mut self,
        account_id: AccountId,
        contract_id: AccountId,
        balance: Balance,
    ) -> U128 {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULT");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"ERR_CALLBACK"),
            PromiseResult::Successful(_value) => {
                let mut account = self.accounts.get(&account_id).unwrap();
                account.reset_reward(&contract_id);
                self.accounts.insert(&account_id, &account);

                let mut staking_pool = self
                    .staking_pools
                    .get(&contract_id)
                    .expect("Pool not found");
                staking_pool.total_paid_reward_balance += balance;
                self.staking_pools.insert(&contract_id, &staking_pool);
                U128(balance)
            }
        }
    }

    //Stake user token
    #[private]
    pub fn ft_transfer_callback(
        &mut self,
        amount: U128,
        account_id: AccountId,
        contract_id: AccountId,
    ) -> U128 {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULT");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"ERR_CALLBACK"),
            PromiseResult::Successful(_value) => {
                let upgradable_account = self.accounts.get(&account_id).unwrap();
                let mut account = Account::from(upgradable_account);

                account.reset_reward(&contract_id);
                self.accounts.insert(&account_id, &account);

                let mut staking_pool = self
                    .staking_pools
                    .get(&contract_id)
                    .expect("Pool not found");
                staking_pool.total_paid_reward_balance += amount.0;
                self.staking_pools.insert(&contract_id, &staking_pool);
                amount
            }
        }
    }

    #[private]
    pub fn ft_withdraw_callback(
        &mut self,
        account_id: AccountId,
        contract_id: AccountId,
        balance: Balance,
    ) -> U128 {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");

        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_value) => {
                //NOTE: Update global pool
                let mut staking_pool = self
                    .staking_pools
                    .get(&contract_id.clone())
                    .expect("Pool not found");
                let new_contract_reward =
                    self.internal_calculate_global_reward(contract_id.clone());
                staking_pool.pre_reward += new_contract_reward;
                staking_pool.last_block_balance_change = env::block_index();
                staking_pool.total_stake_balance -= balance;
                self.staking_pools
                    .insert(&contract_id.clone(), &staking_pool);
                U128(balance)
            }
            PromiseResult::Failed => {
                //NOTE: handle rollback data
                let mut account = self.accounts.get(&account_id).expect("User not found");
                account.add_stake(&contract_id, balance);
                self.accounts.insert(&account_id, &account);

                U128(0)
            }
        }
    }
}
