use anchor_lang::prelude::*;

declare_id!("Staking1111111111111111111111111111111111111");

#[program]
pub mod stablecoin_staking {
    use super::*;
    pub fn placeholder(ctx: Context<Init>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init {}
