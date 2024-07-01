
#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("BWxM86oVK8VW6NMA7guRLKB96ejYcLxiZUeKSF8Fsh49");

#[program]
pub mod counter_anchor {
    use super::*;

    pub fn initialize_counter(_ctx: Context<InitializeCounter>) -> Result<()> {
        Ok(())
    }

    pub fn updateDetails(ctx: Context<UpdateData>,studentName:String,studentId:u16 , address:String) -> Result<()> {
        ctx.accounts.studentDetails.studentId=studentId;
        ctx.accounts.studentDetails.name=studentName;
        ctx.accounts.studentDetails.address=address;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space = 9000,
        payer = payer
    )]
    pub studentDetails: Account<'info, StudentDetails>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(mut)]
    pub studentDetails: Account<'info, StudentDetails>,
}

#[account]
#[derive(InitSpace)]
pub struct StudentDetails {
    studentId:u16,
     #[max_len(50)] 
    name: String,
     #[max_len(50)] 
    address: String,

}
