use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(water_external_id: String, energy_external_id: String)] // Receive the meter IDs
pub struct ReceiveEnvironmentData<'info> {
    pub owner: Signer<'info>,
    #[account(
            mut,
            seeds = [b"user_data", owner.key().as_ref()],
            bump,
            has_one = owner
        )]
    pub user_data: Account<'info, User>,
    #[account(
            mut,
            seeds = [b"property", user_data.owner.key().as_ref(), property.property_external_id.as_bytes()],
            bump,
            has_one = owner
        )]
    pub property: Account<'info, Property>,
    #[account(
        mut,
        has_one = property,
        constraint = water_meter_account.water_external_id == water_external_id, // Match the incoming ID
        seeds = [b"water_meter", user_data.owner.key().as_ref(), property.property_external_id.as_bytes()],
        bump
    )]
    pub water_meter_account: Account<'info, WaterMeter>,

    #[account(
        mut,
        has_one = property,
        constraint = energy_meter_account.energy_external_id == energy_external_id, // Match the incoming ID
        seeds = [b"energy_meter", user_data.owner.key().as_ref(), property.property_external_id.as_bytes()],
        bump
    )]
    pub energy_meter_account: Account<'info, EnergyMeter>,

    #[account(
            mut,
            has_one = owner
        )]
    pub reward_account: Account<'info, UserReward>,
}

impl<'info> ReceiveEnvironmentData<'info> {
    pub fn receive_water_usage(&mut self, usage_amount: u64) -> Result<()> {
        let baseline_usage =
            Self::calculate_baseline_water(&self.water_meter_account.usage_history);

        self.water_meter_account
            .usage_history
            .push(WaterUsageRecord {
                timestamp: Clock::get()?.unix_timestamp,
                amount: usage_amount,
                baseline_usage,
            });
        self.water_meter_account.last_calculated_timestamp = Clock::get()?.unix_timestamp;
        self.water_meter_account.total_water_consumed += usage_amount;
        let total_save_this_round: u64 = baseline_usage.checked_sub(usage_amount).unwrap_or(0);
        if total_save_this_round > 0 {
            self.water_meter_account.total_water_saved += total_save_this_round;
        }
        self.reward_account.total_reward_balance +=
            Self::calculate_points(baseline_usage, usage_amount);
        Ok(())
    }

    pub fn receive_energy_consumption(&mut self, usage_amount: u64) -> Result<()> {
        let baseline_usage =
            Self::calculate_baseline_energy(&self.energy_meter_account.consumption_history);
        self.energy_meter_account
            .consumption_history
            .push(EnergyConsumptionRecord {
                timestamp: Clock::get()?.unix_timestamp,
                amount: usage_amount,
                baseline_usage,
            });
        self.energy_meter_account.last_calculated_timestamp = Clock::get()?.unix_timestamp;

        self.energy_meter_account.total_energy_consumed += usage_amount;
        let total_save_this_round: u64 = baseline_usage.checked_sub(usage_amount).unwrap_or(0);
        if total_save_this_round > 0 {
            self.energy_meter_account.total_energy_saved += total_save_this_round;
        }

        self.reward_account.total_reward_balance +=
            Self::calculate_points(baseline_usage, usage_amount);
        Ok(())
    }

    fn calculate_baseline_water(usage_history: &Vec<WaterUsageRecord>) -> u64 {
        // (Same baseline calculation as before)
        let last_6_months = usage_history.iter().rev().take(6);
        let mut total_usage = 0;
        let mut count = 0;
        for record in last_6_months {
            total_usage += record.amount;
            count += 1;
        }
        if count > 0 {
            total_usage / count
        } else {
            0
        }
    }

    fn calculate_baseline_energy(usage_history: &Vec<EnergyConsumptionRecord>) -> u64 {
        // (Same baseline calculation as before)
        let last_6_months = usage_history.iter().rev().take(6);
        let mut total_usage = 0;
        let mut count = 0;
        for record in last_6_months {
            total_usage += record.amount;
            count += 1;
        }
        if count > 0 {
            total_usage / count
        } else {
            0
        }
    }

    fn calculate_points(baseline_usage: u64, current_usage: u64) -> u64 {
        // (Same points calculation as before)
        if baseline_usage == 0 {
            return 0;
        }
        let reduction_percentage =
            (baseline_usage.saturating_sub(current_usage)) as f64 / baseline_usage as f64 * 100.0;

        if reduction_percentage >= 16.0 {
            100
        } else if reduction_percentage >= 11.0 {
            50
        } else if reduction_percentage >= 6.0 {
            25
        } else if reduction_percentage >= 1.0 {
            10
        } else {
            0
        }
    }
}
