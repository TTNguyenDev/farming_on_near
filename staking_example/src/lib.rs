use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::{env, AccountId, Balance, BlockHeight, EpochHeight, near_bindgen, PanicOnDefault, BorshStorageKey, Promise, PromiseOrValue};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Timestamp;

use crate::config::*;
use crate::account::*;
pub use crate::account::*;
use crate::util::*;

mod config;
mod account;
mod util;
mod internal;
mod enumeration;
mod core_impl;

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
pub struct StakingContract {
    pub owner_id: AccountId,
    pub ft_contract_id: AccountId,
    pub config: Config, // cấu hình công thức trả thưởng cho user,
    pub total_stake_balance: Balance,
    pub total_paid_reward_balance: Balance,
    pub total_staker: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, Account>, // thông tin chi tiết của acount map theo account id
    pub new_data: U128
    //TODO: Implement a state for contract => Allow admin pause contract anytime
}

#[near_bindgen]
impl StakingContract {
    #[init]
    pub fn new(owner_id: AccountId, ft_contract_id: AccountId, config: Config) -> Self {
        StakingContract {
            owner_id,
            ft_contract_id,
            config,
            total_stake_balance: 0,
            total_paid_reward_balance: 0,
            total_staker: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            accounts: LookupMap::new(StorageKey::AccountKey),
            new_data: U128(0)
        }
    }

    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) {
        assert_at_least_one_yocto();
        let account = account_id.unwrap_or_else(|| env::predecessor_account_id());
        let account_stake = self.accounts.get(&account);

        if account_stake.is_some() {
            // refund toàn bộ token deposit
            refund_deposit(0);
        } else {
            // Tạo account mới
            let before_storage_usage = env::storage_usage();
            self.internal_register_account(account.clone());
            let after_storage_usage = env::storage_usage();
            // Refund lại token deposit còn thừa
            refund_deposit(after_storage_usage - before_storage_usage);
        }
    }

    pub fn storage_balance_of(&self, account_id: AccountId) -> U128 {
        let account = self.accounts.get(&account_id);

        if account.is_some() {
            U128(1)
        } else {
            U128(0)
        }
    }

    pub fn get_new_data(&self) -> U128 {
        self.new_data
    }
}

