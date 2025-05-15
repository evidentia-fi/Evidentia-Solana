use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, MintTo};

declare_id!("BondToKEN11111111111111111111111111111111111");

#[program]
pub mod bond_tokenization {
    use super::*;

    pub fn mint_bond(ctx: Context<MintBond>, isin: String) -> Result<()> {
        require!(isin.len() <= 12, ErrorCode::InvalidISINLength);
        let bond = &mut ctx.accounts.bond_metadata;
        bond.isin = isin;
        bond.mint = ctx.accounts.mint.key();
        bond.authority = ctx.accounts.authority.key();

        mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(isin: String)]
pub struct MintBond<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 12)]
    pub bond_metadata: Account<'info, BondMetadata>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    /// CHECK: Authority is validated via CPI
    pub mint_authority: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BondMetadata {
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub isin: String,
}

#[error_code]
pub enum ErrorCode {
    #[msg("ISIN number must be 12 characters or less.")]
    InvalidISINLength,
}
