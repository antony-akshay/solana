use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("2gAvcnmos8XaEJSsVteAS3xxx6WRHpoDziKgb1garZPy");

const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod vesting {
    use super::*;

    //initialise vesting contract
    //give employer ability to add employees
    //allow employees to claim the vested tokens

    pub fn create_vesting_account(
        ctx: Context<CreateVestingAccount>,
        company_name: String,
    ) -> Result<()> {
        *ctx.accounts.vesting_account = VestingAccount {
            owner: ctx.accounts.signer.key(),
            mint: ctx.accounts.mint.key(),
            treasury_token_account: ctx.accounts.treasury_token_account.key(),
            company_name,
            treasury_bump: ctx.bumps.treasury_token_account,
            bump: ctx.bumps.vesting_account,
        };
        Ok(())
    }

    pub fn create_employee_account(ctx:Context<CreateEmployeeAccount>,start_time:i64,end_time:i64,total_amount:u64,cliff_time:i64) -> Result<()> {
        *ctx.accounts.employee_account = EmployeeAccount{ 
            beneficery: ctx.accounts.beneficery.key(), 
            start_time: start_time, 
            end_time: end_time,
            cliff_time: cliff_time,
            vesting_account: ctx.accounts.vesting_account.key(),
            total_amount: total_amount,
            total_withdraw: 0 ,
            bump:ctx.bumps.employee_account
        };
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(company_name:String)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer=signer,
        space= ANCHOR_DISCRIMINATOR_SIZE + VestingAccount::INIT_SPACE,
        seeds = [company_name.as_ref()],
        bump
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        token::mint = mint,
        token::authority = treasury_token_account,
        seeds = [b"vesting_treasury",company_name.as_bytes()],
        bump
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub treasury_token_account: Pubkey,
    #[max_len(50)]
    pub company_name: String,
    pub treasury_bump: u8,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
    pub beneficery: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub vesting_account: Pubkey,
    pub total_amount: u64,
    pub total_withdraw: u64,
    pub bump:u8
}



#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info>{
    #[account(mut)]
    pub owner:Signer<'info>,

    pub beneficery:SystemAccount<'info>,

    #[account(
        has_one=owner //owner if vesting acc is the signer of this instruction
    )]
    pub vesting_account:Account<'info,VestingAccount>,

    #[account(
        init,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR_SIZE + EmployeeAccount::INIT_SPACE,
        seeds=[b"employee_vesting",beneficery.key().as_ref(),vesting_account.key().as_ref()],
        bump
    )]
    pub employee_account:Account<'info,EmployeeAccount>,

    pub system_program:Program<'info,System>
}