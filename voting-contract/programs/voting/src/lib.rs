use anchor_lang::prelude::*;

declare_id!("2irGfjP8e58Lhks8K1EBDyurdMELB1emJqEfVUkLp4uq");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_ends: u64,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll_account;
        poll.poll_id = poll_id;
        poll.description = description;
        poll.poll_start = poll_start;
        poll.poll_end = poll_ends;
        poll.candidate_amount = 0;
        Ok(())
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        candidate_name: String,
        _poll_id: u64,
    ) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate_account;
        let poll = &mut ctx.accounts.poll_account;
        poll.candidate_amount += 1;
        candidate.candidate_name = candidate_name;
        candidate.candidaite_votes = 0;
        Ok(())
    }

    pub fn vote(
        ctx: Context<Vote>, 
        _candidate_name: String, 
        _poll_id: u64) -> Result<()> {
        let candidate = &mut ctx.accounts.candidate_account;
        candidate.candidaite_votes += 1;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id:u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer=signer,
        seeds=[poll_id.to_be_bytes().as_ref()],
        bump,
        space= 8 + PollAccount::INIT_SPACE

    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    pub poll_id: u64,
    #[max_len(280)]
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}

#[derive(Accounts)]
#[instruction(candidate_name:String,poll_id:u64)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[poll_id.to_be_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer=signer,
        space=8+CandidateAccount::INIT_SPACE,
        seeds=[poll_id.to_be_bytes().as_ref(),candidate_name.as_bytes()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidaite_votes: u64,
}

#[derive(Accounts)]
#[instruction(candidate_name:String,poll_id:u64)]

pub struct Vote<'info> {
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[poll_id.to_be_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds=[poll_id.to_be_bytes().as_ref(),candidate_name.as_bytes()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
}
