use anchor_lang::prelude::*;

declare_id!("3aJwM3ioqjqmSSTmqtisuLaZgWvTmgxbpR81gdEC43uz");

#[program]
mod array_example {
    use super::*;

    pub fn initialize(ctx: Context<SumArray>, numbers: Vec<u32>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = numbers;
        my_account.sum_of_data = 0; // Initialize the sum to zero
        Ok(())
    }

    pub fn sum_all_numbers(ctx: Context<CalculationSum>) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        let mut sum = 0;
        for &num in my_account.data.iter() {
            sum += num;
        }
        my_account.sum_of_data = sum;
        Ok(())
    }

    #[derive(Accounts)]
    pub struct SumArray<'info> {
        #[account(mut)]
        pub payer: Signer<'info>,

        #[account(
            init,
            payer = payer,
            space = 8 + 4 + 100 * 4 + 4 // Adjust space: 8 for discriminator, 4 for vec len, 100*4 for 100 u32s, 4 for sum
        )]
        pub my_account: Account<'info, MyAccount>,

        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct CalculationSum<'info> {
        #[account(mut)]
        pub my_account: Account<'info, MyAccount>,
    }

    #[account]
    pub struct MyAccount {
        pub data: Vec<u32>,
        pub sum_of_data: u32,
    }
}
