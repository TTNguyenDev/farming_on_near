use crate::*;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedPool {
    pub total_stake_balance: U128,
    pub total_reward: U128,
    pub total_stakers: U128,
    pub total_paid_reward_balance: U128,
}

#[near_bindgen]
impl StakingContract {
    pub fn get_account_info(&self, account_id: AccountId) -> WrappedAccount {
        let upgradable_account = self.accounts.get(&account_id).unwrap();

        let account = Account::from(upgradable_account);
        let new_reward = self.internal_calculate_account_reward(&account);
        WrappedAccount::from(account_id.clone(), new_reward, account)
    }

    pub fn get_account_reward(&self, account_id: AccountId) -> Balance {
        let upgradable_account = self.accounts.get(&account_id).unwrap();

        let account = Account::from(upgradable_account);
        let new_reward = self.internal_calculate_account_reward(&account);

        account.pre_reward + new_reward
    }

    pub fn get_pool_info(&self) -> WrappedPool {
        WrappedPool {
            total_stake_balance: U128(self.total_stake_balance),
            total_reward: U128(self.pre_reward + self.internal_calculate_global_reward()),
            total_stakers: U128(self.total_staker),
            total_paid_reward_balance: U128(self.total_paid_reward_balance),
        }
    }
}
