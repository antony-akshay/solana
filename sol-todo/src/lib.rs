use anchor_lang::prelude::*;

pub mod error;
pub mod constant;
pub mod states;

use crate::{constant::*,states::*,error::*};

declare_id!("");

pub mod todo_sol{
    use super::*;
    //initialse user
    //add user profile to chain
    //add values for default data
    pub fn initialize_user(ctx:Context<InitializeUser>) ->Result<()>{
        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.authority=ctx.accounts.authority.key();
        user_profile.last_todo=0;
        user_profile.todo_count=0;

        Ok(())
    }

    //add  a todo to chain
    //mark a todo
    //remove data from chain
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info>{

    #[acccount(mut)]
    pub authority:Signer<'info>

    #[account(
        init,
        seeds=[USER_TAG,authority.key().as_ref()],
        bump,
        payer=authority,
        space = std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info,UserProfile>>  //box is a space in memory holding the data

    pub system_program:Program<'info,System>

}