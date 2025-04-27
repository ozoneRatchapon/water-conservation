use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub owner: Pubkey,                 // 1. Owner of the account (user's wallet)
    pub property_account: Vec<Pubkey>, // 2. Property account associated with the user
    pub reward_account: Pubkey,        // 3. Reward account associated with the user
    pub registration_timestamp: i64,   // 4. Timestamp of when the user registered
    pub bump: u8,                      // 5. Bump seed for the account
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
    // space for bump seed is 1 byte
    const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 32 + 8 + 8 + 32 + 8 + 1;
}
