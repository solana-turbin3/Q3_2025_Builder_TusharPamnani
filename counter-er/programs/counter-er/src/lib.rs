#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::{commit, delegate, ephemeral};
use ephemeral_rollups_sdk::cpi::DelegateConfig;
use ephemeral_rollups_sdk::ephem::{commit_accounts, commit_and_undelegate_accounts};

declare_id!("5Z3AeNMmMkERAmbWrohELnwgn4usjK6k85hdWs4yrqcq");

pub const TEST_PDA_SEED: &[u8] = b"test-pda";

#[ephemeral]
#[program]
pub mod counter_er {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;

        if counter.count > 10000 {
            counter.count = 0;
        }

        Ok(())
    }

    pub fn delegate(ctx: Context<DelegateCounter>) -> Result<()> {
        ctx.accounts.delegate_counter(
            &ctx.accounts.payer,
            &[TEST_PDA_SEED],
            DelegateConfig::default(),
        )?;
        Ok(())
    }

    pub fn commit(ctx: Context<IncrementAndCommit>) -> Result<()> {
        commit_accounts(
            &ctx.accounts.payer,
            vec![&ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;

        Ok(())
    }

    pub fn increment_and_commit(ctx: Context<IncrementAndCommit>) -> Result<()> {

        let counter = &mut ctx.accounts.counter;
        counter.count += 1;

        commit_accounts(
            &ctx.accounts.payer,
            vec![&ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;

        Ok(())
    }

    pub fn undelegate(ctx: Context<IncrementAndCommit>) -> Result<()> {
        commit_and_undelegate_accounts(
            &ctx.accounts.payer,
            vec![&ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;

        Ok(())
    }

    pub fn increment_and_undelegate(ctx: Context<IncrementAndCommit>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;

        counter.exit(&crate::ID)?;

        commit_and_undelegate_accounts(
            &ctx.accounts.payer,
            vec![&ctx.accounts.counter.to_account_info()],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program,
        )?;

        Ok(())
    }
}


#[derive(Accounts)]

pub struct Initialize<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 8,
        seeds = [TEST_PDA_SEED],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [TEST_PDA_SEED],
        bump,
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>,
}


#[account]
pub struct Counter {
    pub count: u64
}


#[delegate]
#[derive(Accounts)]
pub struct DelegateCounter<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut, del, seeds = [TEST_PDA_SEED], bump,
    )]
    pub counter: Account<'info, Counter>,

}

#[commit]
#[derive(Accounts)]
pub struct IncrementAndCommit<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [TEST_PDA_SEED], bump,)]
    pub counter: Account<'info, Counter>,

}