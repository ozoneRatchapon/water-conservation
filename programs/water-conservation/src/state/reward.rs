use anchor_lang::prelude::*;

#[account]
pub struct UserReward {
    pub owner: Pubkey,
    pub total_reward_balance: u64,
    pub redemption_history: Vec<RedemptionRecord>,
    pub bump: u8, // Bump seed for the account
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RedemptionRecord {
    pub timestamp: i64,
    pub amount: u64,
}

impl Space for UserReward {
    const INIT_SPACE: usize = 8 + 32 + 8 + 4 + 100 + 1;
}
