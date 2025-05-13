use anchor_lang::prelude::*;

#[account]
pub struct WaterMeter {
    pub property: Pubkey,                     // Link to the property account
    pub water_meter_account: Pubkey,          // Link to the water meter accounts
    pub water_external_id: String,            // from government
    pub usage_history: Vec<WaterUsageRecord>, //
    pub last_calculated_timestamp: i64,
    pub depin_feed_address: Pubkey, // Switchboard feed address
    pub total_water_saved: u64,
    pub total_water_consumed: u64,
    pub bump: u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct WaterUsageRecord {
    pub timestamp: i64,
    pub amount: u64,
    pub baseline_usage: u64,
}

impl Space for WaterMeter {
    // space for property_external_id string is 32
    // space for water_meter_accounts Pubkey is 32
    // space for water_external_id String is 32
    // space for usage_history Vec<WaterUsageRecord> is 32
    // space for last_calculated_timestamp i64 is 8
    // space for depin_feed_address Pubkey is 32
    // space for total_water_saved u64 is 8
    // space for total_water_consumed u64 is 8
    // space for bump u8 is 1
    const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 32 + 8 + 32 + 8 + 8 + 1;
}
