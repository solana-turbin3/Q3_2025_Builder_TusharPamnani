// This file defines the 'Refund' instruction for the escrow program.
// It handles the process where the maker cancels the escrow offer:
// - The maker reclaims their escrowed Token A if no taker has accepted the offer.
// - All Token A is transferred from the vault (escrow PDA) back to the maker.
// - The vault is closed and rent is returned to the maker.
//
// Key roles:
// - 'maker': The user who created the escrow offer and is reclaiming their tokens.
// - 'vault': The escrow PDA's associated token account holding Token A.
// - Only the maker can call this instruction, and only if the offer has not been taken.

use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use crate::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    /// The maker (person who created the escrow offer and is reclaiming their tokens).
    #[account(mut)]
    pub maker: Signer<'info>,

    /// The mint of the token being refunded (Token A).
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    /// The maker's associated token account for Token A (destination for refund).
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    /// The escrow state account (PDA) holding offer details.
    /// - 'close = maker': Closes the account and sends rent to the maker.
    /// - 'has_one = mint_a': Ensures this escrow is for the correct mint.
    /// - 'has_one = maker': Ensures only the maker can refund.
    /// - 'seeds': [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()] ensures uniqueness per offer.
    /// - 'bump': Anchor finds and stores the bump for PDA security.
    #[account(
        mut,
        close = maker,
        has_one = mint_a,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// The vault ATA (PDA) holding Token A, owned by the escrow PDA.
    /// This is the source of Token A for the refund.
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// Standard program accounts required for CPI and ATA creation.
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

}

impl<'info> Refund<'info> {
    /// Called when the maker cancels the escrow (no taker).
    /// Transfers all Token A from the vault (escrow PDA) back to the maker's associated token account for Token A (maker_ata_a).
    /// Closes the vault account, sending the rent to the maker.
    ///
    /// Difference from withdraw_and_close_vault: Here, the maker receives the escrowed tokens back. In withdraw_and_close_vault, the taker receives them on swap success.
    pub fn refund_and_close_vault(&mut self) -> Result<()> {
        // Seeds for signing as the escrow PDA (required to move tokens out of the vault and close it)
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        // Transfer all Token A from the vault to the maker's ATA
        let transfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info()
        };

        let transfer_cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), transfer_accounts, &signer_seeds);

        transfer_checked(transfer_cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        // Close the vault account, sending rent to the maker
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let close_cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), close_accounts, &signer_seeds);

        close_account(close_cpi_ctx)
    }
}