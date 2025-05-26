use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile{
    pub authority:PubKey,
    pub last_todo:u8,
    pub todo_count:u8
}

#[account]
#[derive(Default)]
pub struct TodoAccount{
    pub authority:PubKey,
    pub idx:u8,
    pub content:u8,
    pub marked:bool
}