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
        external_account_id: String,
        depin_feed_address: Pubkey,
    ) -> Result<()> {
        ctx.accounts
            .connect_depin_feed_address(external_account_id, depin_feed_address, &ctx.bumps)
    }
}
