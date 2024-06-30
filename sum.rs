#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("3vMqAm67YBq53RxtS7CUinfrN7qUpVbLdPmvQFaVNMns");

#[program]
pub mod counter_anchor {
    use super::*;

    pub fn initialize_counter(_ctx: Context<InitializeCounter>) -> Result<()> {
        Ok(())
    }

    pub fn calculation(ctx: Context<Calculation>,firstNo:u64 , secondNo:u64) -> Result<()> {
ctx.accounts.sum.sum=firstNo+secondNo;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = 8 + Sum::INIT_SPACE,
        payer = payer
    )]
    pub sum: Account<'info, Sum>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Calculation<'info> {
    #[account(mut)]
    pub sum: Account<'info, Sum>,
}

#[account]
#[derive(InitSpace)]
pub struct Sum {
    sum: u64,
}
