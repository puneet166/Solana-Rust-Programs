#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("5rsX8kPR3EYsFwzoHWpUuYYMMjKKxmZM7QLPjbg2NXx1");

#[program]
pub mod counter_anchor {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        Ok(())
    }

    pub fn update_details(ctx: Context<UpdateData>, student_name: String, student_id: u16, address: String) -> Result<()> {
        let account = &mut ctx.accounts.student_details;
        account.student_id = student_id;
        account.name = student_name;
        account.address = address;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = 8 + StudentDetails::INIT_SPACE, 
        payer = payer
    )]
    pub student_details: Account<'info, StudentDetails>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub student_details: Account<'info, StudentDetails>,
}

#[account]
pub struct StudentDetails {
    student_id: u16,
    name: String,
    address: String,
}

impl StudentDetails {
    const INIT_SPACE: usize = 2 + 4 + 50 + 4 + 50; // size of u16 + lengths for name and address
}
