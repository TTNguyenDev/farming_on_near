use std::collections::HashMap;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
// #[serde(crate = "near_sdk::serde")]
pub struct Account {
    pub stake_balance: HashMap<AccountId, Balance>,
    pub pre_reward: HashMap<AccountId, Balance>,
    pub last_block_balance_change: HashMap<AccountId, BlockHeight>,
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
            assert!(*x >= amount, "Substract overflow");
            *x = *x - amount;

            *self
                .last_block_balance_change
                .get_mut(token)
                .unwrap_or(&mut 0) = env::block_index();
        } else {
            panic!("Stake balance is zero");
        }
    }

    pub(crate) fn get_stake_balance(&self, contract_id: AccountId) -> Balance {
        *self.stake_balance.get(&contract_id).unwrap_or(&0)
    }
}

#[near_bindgen]
impl StakingContract {
    pub fn get_account_reward(&self, account_id: AccountId, token: AccountId) -> Balance {
        let account = self.accounts.get(&account_id).unwrap();

        let new_reward = self.internal_calculate_account_reward(&account, token.clone());

        account.pre_reward.get(&token).unwrap_or(&0) + new_reward
    }
}
