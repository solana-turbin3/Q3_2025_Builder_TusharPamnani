use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer}
};

#[derive(Accounts)]
pub struct Stake<'info> {
    // the user who owns the nft and wants to stke them
    #[account(mut)]
    pub user: Signer<'info>,

    // user's staking account to update the points and stake count
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    // config containing the staking params
    #[account(
        seeds = [b"config", config.key().as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    // the nft mint to be staked
    pub nft_mint: Account<'info, Mint>,

    // user's associated token account
    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub user_nft_ata: Account<'info, TokenAccount>,

    // vault associated token account where all the nfts will be stored
    #[account(
        init_if_needed,
        payer = user,
        seeds = [b"vault", nft_mint.key().as_ref()],
        bump,
        token::mint = nft_mint,
        token::authority = config,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    // stake record PDA to track individaul NFT stake info
    #[account(
        init,
        payer = user,
        seeds = [b"stake", user.key().as_ref(), nft_mint.key().as_ref()], // unique for user + nft mint
        bump,
        space = 8 + StakeAccount::INIT_SPACE,
    )] 
    pub stake_account: Account<'info, StakeAccount>,

    pub token_program: Program<'info, Token>,
    pub associate_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {}