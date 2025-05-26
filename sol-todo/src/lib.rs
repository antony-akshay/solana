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
    pub fn add_todo(ctx:Context<AddTodo>,_content:String) ->Result<()> {
        let todo_account= &mut ctx.accounts.todo_account;
        let user_profile = &mut ctx.accounts.user_profile;

        todo_account.authority=ctx.accounts.authority.key();
        todo_account.content=_content;
        todo_account.marked=false;
        todo_account.idx = user_profile.last_todo;

        //increment user last_todo and todo_count
        user_profile.last_todo = user_profile.last_todo
        .checked_add(1)
        .unwrap();

        user_profile.todo_count = user_profile.todo_count
        .checked_add(1)
        .unwrap();

        Ok(())

    }

    //mark a todo
    pub fn mark_todo(ctx:Context<MarkTodo>,todo_idx:u8) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;
        require!(!todo_account.marked,TodoError::AlreadyMarked);

        todo_account.marked = true;

        Ok(())
    }
    //remove data from chain
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info>{

    #[account(mut)]
    pub authority:Signer<'info>,

    #[account(
        init,
        seeds=[USER_TAG,authority.key().as_ref()],
        bump,
        payer=authority,
        space =8 + std::mem::size_of::<UserProfile>(), //can cause deserialization errors-account descriminator
    )]
    pub user_profile: Box<Account<'info,UserProfile>>,  //box is a space in memory holding the data

    pub system_program:Program<'info,System>

}

#[derive(Accounts)]
#[instruction()]
pub struct AddTodo<'info>{


    #[account(
        mut,
        seeds=[USER_TAG,authority.key().as_ref()],
        bump,
        has_one=authority
    )]
    pub user_profile:Box<Account<'info,UserProfile>>,

    #[account(mut)]
    pub authority:Signer<'info>,

    #[account(
        init,
        seeds=[TODO_TAG,authority.key().as_ref(),&[user_profile.last_todo as u8].as_ref()],
        bump,
        payer=authority,
        space=8 + std::mem::size_of::<TodoAccount>(),
    )]
    pub todo_account:Box<Account<'info,TodoAccount>>,

    pub system_program:Program<'info,System>
}

#[derive(Accounts)]
#[instruction(todo_idx:u8)]
pub struct MarkTodo<'info>{

    #[account(
        mut,
        seeds=[USER_TAG,authority.key().as_ref()],
        bump,
        has_one=authority
    )]
    pub user_profile:Box<Account<'info,UserProfile>>,

    #[account(
        mut,
        seeds=[TODO_TAG,authority.key().as_ref(),&[todo_idx].as_ref()],
        bump,
        has_one=authority
    )]
    pub todo_account:Box<Account<'info,TodoAccount>>,

    #[account(mut)]
    pub authority:Signer<'info>,

    pub system_program : Program<'info,System>
}