use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer,Transfer};

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn deposit(ctx:Context<VaultAction>,amount:u64) ->Result<()> {
        //intented for single deposit only and each vault account is binded to a user's acc
        //check if the vault balance is empty : intitial supposed to be 0
        require_eq!(ctx.accounts.vault.lamports(),0,VaultError::VaultAlreadyExists);
        //check if the the amount is greater enough to meet the rent for the account
        require_gt!(amount,Rent::get()?.minimum_balance(0),VaultError::InvalidAmount);
        
        
        //transfer logic 
        //from user to vaultaccount
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer{
                    from:ctx.accounts.signer.to_account_info(),
                    to:ctx.accounts.vault.to_account_info()
                },
            ),
            amount
        )?;

        Ok(())
    }

    pub fn withdraw(ctx:Context<VaultAction>) -> Result<()>{
        let binding = ctx.accounts.signer.key();
        let signer_seeds = &[b"vault",binding.as_ref(),&[ctx.bumps.vault]];
        
        //PDA has no private key ,so we sign this transcation using seeds used to generate
        //that PDA
        //withdraw the balance from vault to user's account
        //here sender is  a PDA
        //we rederive the PDA here using the seeds

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(), 
                Transfer{
                    from:ctx.accounts.vault.to_account_info(),
                    to:ctx.accounts.signer.to_account_info()
                }, 
                &[&signer_seeds[..]]),
            ctx.accounts.vault.lamports()
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction()]
pub struct VaultAction<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    
    //all controlled by system program
    #[account(
        mut,
        seeds=[b"vault",signer.key().as_ref()],
        bump,
    )]
    pub vault:SystemAccount<'info>,

    pub system_program:Program<'info,System>
}

//custom errors
#[error_code]
pub enum VaultError{
    #[msg("Vault already exists")]
    VaultAlreadyExists,

    #[msg("Invalid amount")]
    InvalidAmount
}

//use cpi to access the system program to run the transaction
