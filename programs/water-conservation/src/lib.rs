#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("CYgCc27FKtjZBwkDjeZ6VfyiifvbdBv9e9Q8Zn872jAK");

#[program]
pub mod water_conservation {
    use super::*;

    pub fn connect_depin_feed_address(
        ctx: Context<ConnectDepin>,
        property_external_id: String,
        water_external_id: String,
        energy_external_id: String,
        water_depin_feed_address: Pubkey,
        energy_depin_feed_address: Pubkey,
        track_energy: bool,
    ) -> Result<()> {
        ctx.accounts.connect_depin_feed_address(
            property_external_id,
            water_external_id,
            energy_external_id,
            water_depin_feed_address,
            energy_depin_feed_address,
            track_energy,
            &ctx.bumps,
        )
    }

    pub fn receive_water_usage(
        ctx: Context<ReceiveEnvironmentData>,
        water_external_id: String,
        usage_amount: u64,
    ) -> Result<()> {
        // Validate the water meter external ID
        require!(
            ctx.accounts.water_meter_account.water_external_id == water_external_id,
            ErrorCode::ConstraintRaw
        );
        ctx.accounts.receive_water_usage(usage_amount)
    }

    pub fn receive_energy_consumption(
        ctx: Context<ReceiveEnvironmentData>,
        energy_external_id: String,
        usage_amount: u64,
    ) -> Result<()> {
        // Validate the energy meter external ID
        require!(
            ctx.accounts.energy_meter_account.energy_external_id == energy_external_id,
            ErrorCode::ConstraintRaw
        );
        ctx.accounts.receive_energy_consumption(usage_amount)
    }

    pub fn redeem_rewards(ctx: Context<RedeemRewards>, reward_amount: u64) -> Result<()> {
        ctx.accounts.redeem_rewards(reward_amount)
    }
}
