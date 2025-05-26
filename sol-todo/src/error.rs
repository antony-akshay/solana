use anchor_lang::prelude::*;

pub enum TodoError{
    #[msg("You are not authorized to perform this action")]
    Unauthorized,

    #[msg("Not allowed")]
    NotAllowed,

    #[msg("Math operation overflow")]
    MathOverflow,

    #[msg("Already marked")]
    AlreadyMarked,
}