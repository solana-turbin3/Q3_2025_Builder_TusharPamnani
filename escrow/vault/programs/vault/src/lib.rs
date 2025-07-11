//! This program lets users create a personal vault account (PDA) to deposit and withdraw SOL securely.
//! Each user has their own vault + state PDA.

#![allow(deprecated)]
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("heiD65tNjyZVxNARhVVsrsa1HPzFThbaxoAmiyV1vzd");

#[program]
pub mod vault {
    use super::*;

    /// Initializes a vault for the calling user.
    /// This creates two PDAs:
    ///     - `vault_state`: stores bump seeds.
    ///     - `vault`: holds SOL.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    /// Deposits `amount` lamports into the user's vault.
    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    /// Withdraws `amount` lamports from the user's vault back to their wallet.
    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    /// Closes the vault by transferring remaining SOL and reclaiming rent.
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
    }
}

/// Initialize context: invoked during `initialize` instruction.
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// The user who pays for account creation and owns the vault.
    #[account(mut)]
    pub user: Signer<'info>,

    /// PDA that stores bump seeds.
    #[account(
        init,
        payer = user,
        space = VaultState::INIT_SPACE,
        seeds = [b"state", user.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    /// PDA that will hold SOL. Created implicitly on chain by PDA logic.
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    /// Handles initialization logic.
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        // Save bump seeds to the state account.
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        Ok(())
    }
}

/// Payment context: used for deposit & withdraw instructions.
#[derive(Accounts)]
pub struct Payment<'info> {
    /// The user who owns the vault.
    #[account(mut)]
    pub user: Signer<'info>,

    /// PDA that holds bump seeds.
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    /// PDA that holds deposited SOL.
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    /// Deposits SOL from user wallet into vault PDA.
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.user.to_account_info(),
                to: self.vault.to_account_info(),
            },
        );
        transfer(cpi_ctx, amount)
    }

    /// Withdraws SOL from vault PDA back to user wallet.
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            Transfer {
                from: self.vault.to_account_info(),
                to: self.user.to_account_info(),
            },
            signer_seeds,
        );
        transfer(cpi_ctx, amount)
    }
}

/// Close context: closes both vault_state PDA and vault PDA.
#[derive(Accounts)]
pub struct Close<'info> {
    /// User who owns the vault.
    #[account(mut)]
    pub user: Signer<'info>,

    /// PDA that stores bump seeds. Closed and rent refunded to user.
    #[account(
        mut,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user
    )]
    pub vault_state: Account<'info, VaultState>,

    /// PDA holding SOL. Transfers all SOL back to user.
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    /// Transfers all SOL from vault PDA to user and closes PDAs.
    pub fn close(&mut self) -> Result<()> {
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            Transfer {
                from: self.vault.to_account_info(),
                to: self.user.to_account_info(),
            },
            signer_seeds,
        );
        transfer(cpi_ctx, self.vault.lamports())
    }
}

/// The PDA that stores bump seeds for this user's vault.
#[account]
pub struct VaultState {
    /// Bump for the `vault` PDA.
    pub vault_bump: u8,

    /// Bump for the `vault_state` PDA itself.
    pub state_bump: u8,
}

impl VaultState {
    /// Space needed for VaultState account.
    /// Anchor discriminator: 8 bytes + 1 byte for each bump.
    pub const INIT_SPACE: usize = 8 + 1 + 1;
}
