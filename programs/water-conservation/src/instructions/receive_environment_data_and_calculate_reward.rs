use crate::errors::GreenmoveError;
use crate::state::*;
use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

#[derive(Accounts)]
pub struct ReceiveEnvironmentData<'info> {
    #[account(
        mut,
        has_one = property
    )]
    pub water_meter_account: Account<'info, WaterMeter>,

    #[account(
        mut,
        has_one = property
    )]
    pub energy_meter_account: Account<'info, EnergyMeter>,

    pub property: Account<'info, Property>,
    pub owner: Signer<'info>,

    #[account(
            mut,
            has_one = owner
        )]
    pub reward_account: Account<'info, UserReward>,
}

impl<'info> ReceiveEnvironmentData<'info> {
    pub fn receive_water_usage(&mut self, amount: u64) -> Result<()> {
        let baseline_usage =
            Self::calculate_baseline_water(&self.water_meter_account.usage_history);
        self.water_meter_account
            .usage_history
            .push(WaterUsageRecord {
                timestamp: Clock::get()?.unix_timestamp,
                amount,
                baseline_usage,
            });
        self.water_meter_account.last_calculated_timestamp = Clock::get()?.unix_timestamp;

        self.reward_account.total_reward_balance += Self::calculate_points(baseline_usage, amount);
        Ok(())
    }

    pub fn receive_energy_consumption(&mut self, amount: u64) -> Result<()> {
        let baseline_usage =
            Self::calculate_baseline_energy(&self.energy_meter_account.consumption_history);
        self.energy_meter_account
            .consumption_history
            .push(EnergyConsumptionRecord {
                timestamp: Clock::get()?.unix_timestamp,
                amount,
                baseline_usage,
            });
        self.energy_meter_account.last_calculated_timestamp = Clock::get()?.unix_timestamp;
        self.reward_account.total_reward_balance += Self::calculate_points(baseline_usage, amount);
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
