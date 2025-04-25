use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub owner: Pubkey,                   // 1. Owner of the account (user's wallet)
    pub water_account_id: String,        // 2. Identifier for the user's external water account
    pub depin_feed_address: Pubkey,      // 3. Address of the DePIN feed providing water data
    pub usage_history: Vec<UsageRecord>, // 4. History of water usage
    pub baseline_usage: u64,             // 5. User's baseline water consumption
    pub reward_token_balance: u64,       // 6. User's Greenmove point balance
    pub redemption_history: Vec<RedemptionRecord>, // 7. History of reward redemptions
    pub registration_timestamp: i64,     // 8. Timestamp of when the user registered
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct UsageRecord {
    pub timestamp: i64,
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct RedemptionRecord {
    pub timestamp: i64,
    pub amount: u64,
    pub reward_type: String,
}

impl Space for User {
    // space for disclaimer is 8 bytes
    // space for the owner is 32 bytes
    // space for the water account ID is 32 bytes
    // space for depin feed address is 32 bytes
    // space for usage history is 32 bytes
    // space for baseline usage is 8 bytes
    // space for reward token balance is 8 bytes
    // space for redemption history is 32 bytes
    // space for registration timestamp is 8 bytes
    const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 32 + 8 + 8 + 32 + 8;
}
