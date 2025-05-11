use crate::state::{EnergyMeter, Property, User, UserReward, WaterMeter};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(property_external_id: String)]
pub struct ConnectDepin<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
            init_if_needed,
            payer = user,
            space = User::INIT_SPACE,
            seeds = [b"user_data", user.key().as_ref()],
            bump
        )]
    pub user_data: Account<'info, User>,

    #[account(
            init,
            payer = user,
            space = Property::INIT_SPACE,
            seeds = [b"property", user.key().as_ref(), property_external_id.as_bytes()],
            bump
        )]
    pub property_account: Account<'info, Property>,

    #[account(
            init,
            payer = user,
            space = WaterMeter::INIT_SPACE,
            seeds = [b"water_meter", user.key().as_ref(), property_external_id.as_bytes()],
            bump
        )]
    pub water_meter_account: Account<'info, WaterMeter>,

    #[account(
            init,
            payer = user,
            space = EnergyMeter::INIT_SPACE,
            seeds = [b"energy_meter", user.key().as_ref(), property_external_id.as_bytes()],
            bump
        )]
    pub energy_meter_account: Account<'info, EnergyMeter>,

    #[account(init,
    payer = user,
    space = UserReward::INIT_SPACE,
    seeds = [b"user_reward", user.key().as_ref()],
    bump)]
    pub reward_account: Account<'info, UserReward>,
    pub system_program: Program<'info, System>,
}

impl ConnectDepin<'_> {
    pub fn connect_depin_feed_address(
        &mut self,
        property_external_id: String,
        water_external_id: String,
        energy_external_id: String,
        water_depin_feed_address: Pubkey,
        energy_depin_feed_address: Pubkey,
        bumps: &ConnectDepinBumps,
    ) -> Result<()> {
        self.user_data.set_inner(User {
            owner: self.user.key(),
            property_account: Vec::from([self.property_account.key()]),
            reward_account: self.reward_account.key(),
            registration_timestamp: Clock::get()?.unix_timestamp,
            bump: bumps.user_data,
        });

        self.property_account.set_inner(Property {
            owner: self.user.key(),
            property_external_id,
            water_meter_accounts: Vec::from([self.water_meter_account.key()]),
            energy_meter_accounts: Vec::from([self.energy_meter_account.key()]),
        });

        self.water_meter_account.set_inner(WaterMeter {
            property: self.property_account.key(),
            water_meter_account: self.water_meter_account.key(),
            water_external_id,
            usage_history: Vec::new(),
            last_calculated_timestamp: 0,
            depin_feed_address: water_depin_feed_address,
            total_water_saved: 0,
            total_water_consumed: 0,
        });

        self.energy_meter_account.set_inner(EnergyMeter {
            property: self.property_account.key(),
            energy_meter_account: self.energy_meter_account.key(),
            energy_external_id,
            consumption_history: Vec::new(),
            last_calculated_timestamp: 0,
            depin_feed_address: energy_depin_feed_address,
            total_energy_saved: 0,
            total_energy_consumed: 0,
        });

        self.reward_account.set_inner(UserReward {
            owner: self.user.key(),
            total_reward_balance: 0,
            redemption_history: Vec::new(),
        });

        Ok(())
    }
}
