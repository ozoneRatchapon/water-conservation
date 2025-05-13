use anchor_lang::prelude::*;

#[account]
pub struct Property {
    pub owner: Pubkey,                      // Owner of the property
    pub property_external_id: String,       // from government
    pub water_meter_accounts: Vec<Pubkey>,  // Vec of WaterMeterAccount addresses
    pub energy_meter_accounts: Vec<Pubkey>, // Vec of EnergyMeterAccount addresses
    pub bump: u8,                           // Bump seed for the account
}

impl Space for Property {
    // space for owner is 32
    // space for property_external_id string is 32
    // space for water_meter_accounts Vec<Pubkey> is 32
    // space for energy_meter_accounts Vec<Pubkey> is 32
    // space for bump seed is 1 byte
    const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 32 + 1;
}
