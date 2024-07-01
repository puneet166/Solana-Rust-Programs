#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("CyczKjN2R9W2iZct9824nNg4xhWsiwU2Ry981FHSDfQB");

#[program]
pub mod counter_anchor {
    use super::*;

    pub fn initialize_counter(ctx: Context<InitializeCounter>) -> Result<()> {
        Ok(())
    }

    pub fn update_details(
        ctx: Context<UpdateData>,
        user_detail: Pubkey,
        student_name: String,
        student_id: u16,
        address: String,
    ) -> Result<()> {
        if ctx.accounts.payer.key() == user_detail {
            let account = &mut ctx.accounts.student_details;
            account.student_id = student_id;
            account.name = student_name;
            account.address = address;
            account.userAddress = ctx.accounts.payer.key();
            Ok(())
        } else {
            Err(MyError::Unauthorized.into())
        }
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + 2 + 32 + 32, // Space for StudentDetails
        
    )]
    pub student_details: Account<'info, StudentDetails>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut
    )]
    pub student_details: Account<'info, StudentDetails>,
}

#[account]
pub struct StudentDetails {
    student_id: u16,
    name: String,
    address: String,
    userAddress:Pubkey,
}

#[error_code]
pub enum MyError {
    #[msg("Unauthorized action.")]
    Unauthorized,
}
