use crate::state::{EnergyMeter, Property, User, UserReward, WaterMeter};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(property_external_id: String, water_external_id: String, energy_external_id: String, track_energy: bool)]
pub struct ConnectDepin<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
            init,
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
            seeds = [b"water_meter", user.key().as_ref(), property_external_id.as_bytes(), water_external_id.as_bytes()],
            bump
        )]
    pub water_meter_account: Account<'info, WaterMeter>,

    #[account(
            init_if_needed,
            payer = user,
            space = EnergyMeter::INIT_SPACE,
            seeds = [b"energy_meter", user.key().as_ref(), property_external_id.as_bytes(), energy_external_id.as_bytes()],
            bump,
            constraint = track_energy
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
        track_energy: bool,
        bumps: &ConnectDepinBumps,
    ) -> Result<()> {
        self.user_data.set_inner(User {
            owner: self.user.key(),
            property_account: Vec::from([self.property_account.key()]),
            reward_account: self.reward_account.key(),
            registration_timestamp: Clock::get()?.unix_timestamp,
            bump: bumps.user_data,
        });

        let mut water_meter_accounts = std::mem::take(&mut self.property_account.water_meter_accounts);
        if !water_meter_accounts.contains(&self.water_meter_account.key()) {
            water_meter_accounts.push(self.water_meter_account.key());
        }
        let mut energy_meter_accounts = std::mem::take(&mut self.property_account.energy_meter_accounts);
        if track_energy && !energy_meter_accounts.contains(&self.energy_meter_account.key()) {
            energy_meter_accounts.push(self.energy_meter_account.key());
        }
        self.property_account.set_inner(Property {
            owner: self.user.key(),
            property_external_id,
            water_meter_accounts,
            energy_meter_accounts,
            bump: bumps.property_account,
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
            bump: bumps.water_meter_account,
        });

        if track_energy {
            self.energy_meter_account.set_inner(EnergyMeter {
                property: self.property_account.key(),
                energy_meter_account: self.energy_meter_account.key(),
                energy_external_id,
                consumption_history: Vec::new(),
                last_calculated_timestamp: 0,
                depin_feed_address: energy_depin_feed_address,
                total_energy_saved: 0,
                total_energy_consumed: 0,
                bump: bumps.energy_meter_account,
            });
        }

        self.reward_account.set_inner(UserReward {
            owner: self.user.key(),
            total_reward_balance: 0,
            redemption_history: Vec::new(),
            bump: bumps.reward_account,
        });

        Ok(())
    }
}
