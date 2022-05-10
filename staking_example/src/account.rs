use std::collections::HashMap;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
// #[serde(crate = "near_sdk::serde")]
pub struct Account {
    pub stake_balance: HashMap<AccountId, Balance>,
    pub pre_reward: HashMap<AccountId, Balance>,
    pub last_block_balance_change: HashMap<AccountId, BlockHeight>,
    // pub unstake_balance: Balance,
    // pub unstake_start_timestamp: Timestamp,
    // pub unstake_available_epoch: EpochHeight,
    // pub new_account_data: U128,
}

impl Account {
    pub(crate) fn add_reward(&mut self, token: &AccountId, amount: Balance) {
        if let Some(x) = self.pre_reward.get_mut(token) {
            *x = *x + amount;
        } else {
            self.pre_reward.insert(token.clone(), amount);
        }
        *self
            .last_block_balance_change
            .get_mut(token)
            .unwrap_or(&mut 0) = env::block_index();
    }

    pub(crate) fn reset_reward(&mut self, token: &AccountId) {
        if let Some(x) = self.pre_reward.get_mut(token) {
            *x = 0;
        } else {
            self.pre_reward.insert(token.clone(), 0);
        }
        *self
            .last_block_balance_change
            .get_mut(token)
            .unwrap_or(&mut 0) = env::block_index();
    }

    pub(crate) fn add_stake(&mut self, token: &AccountId, amount: Balance) {
        if let Some(x) = self.stake_balance.get_mut(token) {
            *x = *x + amount;
        } else {
            self.stake_balance.insert(token.clone(), amount);
        }
        *self
            .last_block_balance_change
            .get_mut(token)
            .unwrap_or(&mut 0) = env::block_index();
    }

    pub(crate) fn sub_stake(&mut self, token: &AccountId, amount: Balance) {
        if let Some(x) = self.stake_balance.get_mut(token) {
            assert!(*x > amount, "Substract overflow");
            *x = *x - amount;

            *self
                .last_block_balance_change
                .get_mut(token)
                .unwrap_or(&mut 0) = env::block_index();
        } else {
            panic!("Substract overflow");
        }
    }

    pub(crate) fn get_stake_balance(&self, contract_id: AccountId) -> Balance {
        *self.stake_balance.get(&contract_id).unwrap_or(&0)
    }
}

// #[derive(Deserialize, Serialize)]
// #[serde(crate = "near_sdk::serde")]
// pub struct WrappedAccount {
//     pub account_id: AccountId,
//     pub stake_balance: U128,
//     pub unstake_balance: U128,
//     pub reward: U128,
//     pub can_withdraw: bool,
//     pub unstake_start_timestamp: Timestamp,
//     pub unstake_available_epoch: EpochHeight,
//     pub current_epoch: EpochHeight,
//     pub new_account_data: U128,
// }

// impl WrappedAccount {
//     pub fn from(account_id: AccountId, new_reward: Balance, account: Account) -> Self {
//         WrappedAccount {
//             account_id,
//             stake_balance: U128(account.stake_balance),
//             unstake_balance: U128(account.unstake_balance),
//             reward: U128(account.pre_reward + new_reward),
//             can_withdraw: account.unstake_available_epoch <= env::epoch_height(),
//             unstake_start_timestamp: account.unstake_start_timestamp,
//             unstake_available_epoch: account.unstake_available_epoch,
//             current_epoch: env::epoch_height(),
//             new_account_data: account.new_account_data,
//         }
//     }
// }
