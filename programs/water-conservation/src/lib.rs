#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use crate::instructions::*;
use crate::state::*;

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
    ) -> Result<()> {
        ctx.accounts.connect_depin_feed_address(
            property_external_id,
            water_external_id,
            energy_external_id,
            water_depin_feed_address,
            energy_depin_feed_address,
            &ctx.bumps,
        )
    }

    pub fn receive_water_usage(ctx: Context<ReceiveEnvironmentData>, amount: u64) -> Result<()> {
        ctx.accounts.receive_water_usage(amount)
    }

    pub fn receive_energy_consumption(
        ctx: Context<ReceiveEnvironmentData>,
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.receive_energy_consumption(amount)
    }

    pub fn redeem_rewards(ctx: Context<RedeemReward>, amount: u64) -> Result<()> {
        ctx.accounts.redeem_rewards(amount)
    }
}
