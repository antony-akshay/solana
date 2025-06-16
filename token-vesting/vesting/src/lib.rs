use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("2gAvcnmos8XaEJSsVteAS3xxx6WRHpoDziKgb1garZPy");

const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod vesting {
    use super::*;

    //initialise vesting contract
    //give emplyee ability to add employees
    //allow employees to claim the vested tokens

    pub fn create_vesting_account(
        ctx: Context<CreateVestingAccount>,
        company_name: String,
    ) -> Result<()> {
        *ctx.accounts.vesting_account = VestingAccount {
            owner: ctx.accounts.signer.key(),
            mint: ctx.accounts.mint.key(),
            treasury_token_account:ctx.accounts.treasury_token_account.key(),
            company_name,
            treasury_bump: ctx.bumps.treasury_token_account,
            bump: ctx.bumps.vesting_account,
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
