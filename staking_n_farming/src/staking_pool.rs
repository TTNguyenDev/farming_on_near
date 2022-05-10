use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StakingPoolInfo {
    pub weight: u16,
    pub total_unstaked_balance: Balance,

    pub total_stake_balance: Balance,
    pub total_paid_reward_balance: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub new_data: U128, //TODO: Implement a state for contract => Allow admin pause contract anytime
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedStakingPoolInfo {
    pub weight: u16,
    pub total_unstaked_balance: WrappedBalance,

    pub total_stake_balance: WrappedBalance,
    pub total_paid_reward_balance: WrappedBalance,
    pub pre_reward: WrappedBalance,
    pub last_block_balance_change: BlockHeight,
    pub new_data: WrappedBalance, //TODO: Implement a state for contract => Allow admin pause contract anytime
}

impl WrappedStakingPoolInfo {
    pub fn from(staking_pool_info: StakingPoolInfo) -> Self {
        WrappedStakingPoolInfo {
            weight: staking_pool_info.weight,
            total_stake_balance: U128::from(staking_pool_info.total_stake_balance),
            total_unstaked_balance: U128::from(staking_pool_info.total_unstaked_balance),
            total_paid_reward_balance: U128::from(staking_pool_info.total_paid_reward_balance),
            pre_reward: U128::from(staking_pool_info.pre_reward),
            last_block_balance_change: staking_pool_info.last_block_balance_change,
            new_data: staking_pool_info.new_data,
        }
    }
}

#[near_bindgen]
impl StakingContract {
    pub fn get_staking_pool_info(&self, contract_id: AccountId) -> WrappedStakingPoolInfo {
        let pool_info = self
            .staking_pools
            .get(&contract_id)
            .expect("Pool not found");
        WrappedStakingPoolInfo::from(pool_info)
    }

    pub fn get_pool_reward(&self, contract_id: AccountId) -> U128 {
        let new_contract_reward = self.internal_calculate_global_reward(contract_id.clone());
        U128(new_contract_reward)
    }
}
