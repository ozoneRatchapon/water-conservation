use anchor_lang::prelude::*;

#[account]
pub struct EnergyMeter {
    pub property: Pubkey,             // Link to the property account
    pub energy_meter_account: Pubkey, // Link to the energy meter account
    pub energy_external_id: String,   // from government
    pub consumption_history: Vec<EnergyConsumptionRecord>, //
    pub last_calculated_timestamp: i64,
    pub depin_feed_address: Pubkey, // Switchboard feed address
    pub total_energy_saved: u64,
    pub total_energy_consumed: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct EnergyConsumptionRecord {
    pub timestamp: i64,
    pub amount: u64,
    pub baseline_usage: u64,
}

impl Space for EnergyMeter {
    // space for property_external_id string is 32
    // space for energy_meter_account Pubkey is 32
    // space for energy_external_id String is 32
    // space for consumption_history Vec<EnergyConsumptionRecord> is 32
    // space for last_calculated_timestamp i64 is 8
    // space for depin_feed_address Pubkey is 32
    // space for total_energy_saved u64 is 8
    // space for total_energy_consumed u64 is 8
    // space for bump u8 is 1
    const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 32 + 8 + 32 + 8 + 8 + 1;
}
