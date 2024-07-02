use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("7Y8KvaXBTpaAFJS2G1mpzUUy4iVSgzrXncsCWbCr6aZm");

#[program]
pub mod lamport_transfer {
    use super::*;

    pub fn transfer_lamports(ctx: Context<TransferLamports>, amount: u64) -> Result<()> {
        let from_account = ctx.accounts.from.to_account_info();
        let to_account = ctx.accounts.to.to_account_info();

        let ix = system_instruction::transfer(
            &from_account.key,
            &to_account.key,
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                from_account.clone(),
                to_account.clone(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
