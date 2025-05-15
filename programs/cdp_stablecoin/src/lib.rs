use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, MintTo};

declare_id!("CDPStab1111111111111111111111111111111111111");

pub const BOND_UNIT_VALUE: u64 = 1000;
pub const MARGIN_PERCENT: u64 = 5;

#[program]
pub mod cdp_stablecoin {
    use super::*;

    pub fn set_borrow_rate(ctx: Context<SetBorrowRate>, new_rate_bps: u64) -> Result<()> {
        ctx.accounts.config.borrow_rate_bps = new_rate_bps;
        Ok(())
    }

    pub fn deposit_bond_and_mint(ctx: Context<DepositBondAndMint>, nft_count: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let config = &ctx.accounts.config;

        require!(ctx.accounts.user_nft_account.amount >= nft_count, ErrorCode::NotEnoughNFTs);

        vault.nft_count += nft_count;
        vault.owner = ctx.accounts.user.key();

        let total_value = BOND_UNIT_VALUE * nft_count;
        let mintable = total_value * (100 - MARGIN_PERCENT) / 100;
        vault.borrowed += mintable;
        vault.last_borrow_timestamp = Clock::get()?.unix_timestamp;

        anchor_spl::token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.stablecoin_mint.to_account_info(),
                    to: ctx.accounts.user_stablecoin_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            mintable,
        )?;

        Ok(())
    }

    pub fn accrue_interest(ctx: Context<AccrueInterest>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let config = &ctx.accounts.config;

        let now = Clock::get()?.unix_timestamp;
        let elapsed = now - vault.last_borrow_timestamp;

        let interest = ((vault.borrowed as u128)
            * (config.borrow_rate_bps as u128)
            * (elapsed as u128))
            / (10000 * 365 * 24 * 3600);

        let interest_u64 = interest as u64;

        anchor_spl::token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.stablecoin_mint.to_account_info(),
                    to: ctx.accounts.staking_reward_vault.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            interest_u64,
        )?;

        vault.last_borrow_timestamp = now;

        Ok(())
    }
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub nft_count: u64,
    pub borrowed: u64,
    pub last_borrow_timestamp: i64,
}

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub borrow_rate_bps: u64,
}

#[derive(Accounts)]
pub struct SetBorrowRate<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, Config>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct DepositBondAndMint<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_stablecoin_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub stablecoin_mint: Account<'info, Mint>,
    /// CHECK: PDA authority
    pub mint_authority: AccountInfo<'info>,
    #[account(init_if_needed, payer = user, space = 8 + 64)]
    pub vault: Account<'info, Vault>,
    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AccrueInterest<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub stablecoin_mint: Account<'info, Mint>,
    #[account(mut)]
    pub staking_reward_vault: Account<'info, TokenAccount>,
    /// CHECK: PDA authority
    pub mint_authority: AccountInfo<'info>,
    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token>,
}
