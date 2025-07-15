use crate::state::*; // add this to use all the values/structs defined in the states directory
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    // the account that will be initializing the config as well as paying for the account creation
    #[account(mut)]
    pub admin: Signer<'info>,

    // the global congif PDA that stores the staking parameters
    #[account(
        init,
        payer = admin,
        seeds=[b"config"],
        bump,
        space = 8 + StakeConfig::INIT_SPACE,
    )]
    pub config: Account<'info, StakeConfig>,

    // the reward token mint - created and owned by the config PDA
    #[account(
        init_if_needed, // create a new mint if not created yet
        payer = admin, // the admin who's paying the rent
        seeds = [b"rewards", config.key().as_ref()], // pda seeds
        bump, // store bump seeds
        mint::decimals = 6, // token decimals
        mint::authority = config, // config owns the mint authority
    )]
    pub rewards_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitializeConfig<'info> {
    // initialize the global config for staking
    pub fn initialize_config(
        &mut self,
        points_per_stake: u8,
        max_stake: u8,
        freeze_perio: u32,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(StakeConfig {
            points_per_stake, // points awarded per nft staked
            max_stake, // max nft allowed to stake at once
            freeze_perio, // staking duration
            rewards_bump: bumps.rewards_mint, // bump for reward mint PDA
            bump: bumps.config, // bump for config PDA
        });

        Ok(())
    }
}
