use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile{
    pub authority:Pubkey, //32
    pub last_todo:u8, //1
    pub todo_count:u8 //1
}

#[account]
#[derive(Default)]
pub struct TodoAccount{
    pub authority:Pubkey,
    pub idx:u8,
    pub content:String,
    pub marked:bool
}
