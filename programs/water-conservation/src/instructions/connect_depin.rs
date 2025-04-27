use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
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
    pub system_program: Program<'info, System>,
}

impl ConnectDepin<'_> {
    pub fn connect_depin_feed_address(
        &mut self,
        external_account_id: String,
        depin_feed_address: Pubkey,
        bumps: &ConnectDepinBumps,
    ) -> Result<()> {
        self.user_data.set_inner(User {
            owner: self.user.key(),
            external_account_id,
            depin_feed_address,
            usage_history: Vec::new(),
            baseline_usage: 0,
            reward_token_balance: 0,
            redemption_history: Vec::new(),
            registration_timestamp: Clock::get()?.unix_timestamp,
            bump: bumps.user_data,
        });
        Ok(())
    }
}
