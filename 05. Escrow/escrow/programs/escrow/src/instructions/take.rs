// This file defines the 'Take' instruction for the escrow program.
// It handles the process where a taker accepts the escrow offer:
// - The taker sends Token B to the maker.
// - The taker receives Token A from the vault (escrow PDA).
//
// Key roles:
// - 'maker': The user who created the escrow offer.
// - 'taker': The user who accepts the offer.
// - 'taker_ata_a': The taker's associated token account for Token A (receives escrowed tokens).
// - 'vault': The escrow PDA's associated token account holding Token A.
//
// Note: 'maker' and 'taker_ata_a' are NOT the same entity. The maker is the offer creator; taker_ata_a is the taker's account that receives Token A when the offer is accepted.

use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, transfer_checked, Mint, TokenAccount, TokenInterface, CloseAccount, TransferChecked},
};

use crate::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    /// The taker (person accepting the offer).
    #[account(mut)]
    pub taker: Signer<'info>,

    /// The maker (person who created the escrow offer).
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    
    /// The mint of the token the taker will receive (Token A).
    pub mint_a: InterfaceAccount<'info, Mint>,
    
    /// The mint of the token the taker will send to the maker (Token B).
    pub mint_b: InterfaceAccount<'info, Mint>,

    /// The taker's associated token account for Token A.
    /// This is where the taker will receive the escrowed tokens.
    ///
    /// Note: 'taker_ata_a' belongs to the taker, not the maker.
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a, 
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>, // ATA for mint_a belonging to taker
    
    /// The taker's associated token account for Token B (from which they'll pay).
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>, // taker ATA from which they'll pay in mint_b
    
    /// The maker's associated token account for Token B (destination for mint_b).
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>, // destination for mint_b 
    
    /// The escrow state account (PDA) holding offer details.
    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    escrow: Account<'info, Escrow>,
    
    /// The vault ATA (PDA) holding Token A, owned by the escrow PDA.
    /// This is the source of Token A for the taker.
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    /// Transfers the expected amount of Token B from the taker to the maker.
    pub fn transfer_to_maker(&mut self) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals) //transfers the expected receive amount of token B
    }

    /// Called when the taker accepts the escrow offer (swap success).
    /// Transfers all Token A from the vault (escrow PDA) to the taker's associated token account for Token A (taker_ata_a).
    /// Closes the vault account, sending the rent to the maker.
    ///
    /// Difference from refund_and_close_vault: Here, the taker receives the escrowed tokens. In refund_and_close_vault, the maker receives them back on cancellation.
    pub fn withdraw_and_close_vault(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]]; // this lets the escrow PDA sign for the transfer from vault

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(), // taker's ATA for Token A
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds,
        );

        transfer_checked(ctx, self.vault.amount, self.mint_a.decimals)?; // transfers all tokens from the vault to the taker        

        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(), // rent goes to maker of the account
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            &signer_seeds,
        );

        close_account(ctx) // rent returns to taker and removes the vault account
    }
}