// This file defines the 'Make' instruction for the escrow program.
// It handles initializing a new escrow offer and the vault, and provides a deposit helper.
//
// The 'Make' instruction is called by the maker to create an escrow offer:
// - The maker specifies what they are offering (mint_a) and what they want in return (mint_b).
// - The escrow PDA and vault ATA are created deterministically using seeds.
// - Only the offered token (mint_a) is ever escrowed in the vault.

use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use crate::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    /// The user creating the escrow offer (the "maker").
    /// Must sign and pay for account creation and rent.
    #[account(mut)]
    pub maker: Signer<'info>,

    /// The mint of the token the maker is offering (Token A).
    /// Used to create the vault and validate transfers.
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    /// The mint of the token the maker wants in return (Token B).
    /// Only stored in escrow state for reference; not escrowed.
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    /// The maker's associated token account for mint_a (Token A).
    /// Source of tokens to be escrowed.
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    
    /// The escrow state account (PDA).
    /// Stores all offer details and is uniquely derived from the maker and a user-supplied seed.
    /// - 'init': Creates the account.
    /// - 'payer': Maker pays for rent.
    /// - 'seeds': [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()] ensures uniqueness per offer.
    /// - 'bump': Anchor finds and stores the bump for PDA security.
    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = 8 + Escrow::INIT_SPACE,
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    /// The vault ATA (PDA) for mint_a, owned by the escrow PDA.
    /// Holds the escrowed tokens until the offer is taken or refunded.
    /// - 'init': Creates the ATA.
    /// - 'payer': Maker pays for rent.
    /// - 'associated_token::authority': escrow PDA is the owner.
    #[account(
        init,
        payer = maker,
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

impl<'info> Make<'info> {
    /// Initializes the escrow state with all offer details.
    /// This function is called by the 'make' instruction handler.
    pub fn init_esrow(&mut self, seed: u64, receive: u64, bumps: &MakeBumps) -> Result<()> {
        // Store all relevant offer info in the escrow state.
        self.escrow.set_inner(
            Escrow { 
                seed, 
                maker: self.maker.key(), 
                mint_a: self.mint_a.key(), 
                mint_b: self.mint_b.key(), 
                receive, 
                bump: bumps.escrow 
            });
        Ok(())
    }

    /// Transfers the specified amount of Token A from the maker's ATA to the vault.
    /// Only the maker needs to sign; no PDA signer_seeds are required for deposit.
    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info()
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer_checked(cpi_ctx, deposit, self.mint_a.decimals)
    }
}