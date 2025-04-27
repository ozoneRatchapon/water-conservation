use crate::errors::GreenmoveError;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RedeemRewards<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner
    )]
    pub reward_account: Account<'info, UserReward>,
    pub system_program: Program<'info, System>,
}

impl<'info> RedeemRewards<'info> {
    pub fn redeem_rewards(&mut self, reward_amount: u64) -> Result<()> {
        let reward_account = &mut self.reward_account;

        if reward_account.total_reward_balance < reward_amount {
            return Err(error!(GreenmoveError::InsufficientPoints));
        }

        reward_account.total_reward_balance -= reward_amount;
        reward_account.redemption_history.push(RedemptionRecord {
            amount: reward_amount,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}
