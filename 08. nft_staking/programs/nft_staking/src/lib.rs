pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use errors::*;

declare_id!("BZ9CvYyMP6yHpE5DxDgPg9pXjq2U7u4jb6tEXjotcu29");


#[program]
pub mod staking {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_unstake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .initialize_config(points_per_stake, max_unstake, freeze_period, ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim_rewards(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}