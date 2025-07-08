#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_lang::{system_program::{Transfer, transfer}};

declare_id!("ERbP2r44adnwsraYoQ3zntdkgiNtLUn1cay6wnxtpjkq");

#[program]
pub mod vault {
    use super::*;

    // this will initialize the accounts and the program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Payment> , amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        mut
    )]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = VaultState::INIT_SPACE,
        seeds = [b"state", user.key().as_ref()],
        bump

    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>

}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,


    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump= vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpl_program: AccountInfo<'_> = self.system_program.to_account_info();

        let cpl_accounts: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        let cpl_ctx = CpiContext::new(cpl_program, cpl_accounts);

        transfer(cpl_ctx, amount)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()>{
        let cpl_program: AccountInfo<'_> = self.system_program.to_account_info();

        let cpl_accounts: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        let seeds: &[&[u8]; 3 ] = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds: &[&[&[u8]]; 1 ] = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpl_program, cpl_accounts, signer_seeds);

        transfer(cpi_ctx, amount)
    }
}



impl<'info> Initialize<'info>{
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()>{
        let rent_exempt: u64 = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());

        let cpl_program: AccountInfo<'_> = self.system_program.to_account_info();

        let cpl_accounts: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info()
        };

        let cpl_ctx= CpiContext::new(cpl_program, cpl_accounts);

        transfer(cpl_ctx, rent_exempt)?;

        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump= bumps.vault_state;

        Ok(())   
    }
}

#[account]
// #[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1*2;
}