use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::utils::assert_one_yocto;
use near_sdk::Timestamp;

use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BlockHeight, BorshStorageKey, EpochHeight,
    Gas, PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};

use crate::account::*;
pub use crate::account::*;
use crate::config::*;

mod account;
mod config;
mod constants;
mod core_impl;
mod enumeration;
mod internal;

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey,
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
    pub new_data: U128, //TODO: Implement a state for contract => Allow admin pause contract anytime
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
            new_data: U128(0),
        }
    }

    #[payable]
    pub fn storage_deposit(&mut self) {
        assert!(
            env::attached_deposit() >= 1,
            "Requires attached deposit of at least 1 yoctoNEAR"
        );

        let account = env::predecessor_account_id();
        let account_stake = self.accounts.get(&account);

        if account_stake.is_some() {
            panic!("Already registered");
        } else {
            let before_storage_usage = env::storage_usage();
            self.internal_register_account(account.clone());
            let after_storage_usage = env::storage_usage();
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

//NOTE: Utils
pub(crate) fn refund_deposit(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();

    assert!(
        attached_deposit >= required_cost,
        "Must attach {} yoctoNear to cover storage",
        required_cost
    );

    let refund = attached_deposit - required_cost;

    if refund > 0 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}
