use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("2syvEMHMfZKWBkHajEKvXtqScPM1huKKdx7LCMwvwRdG");

#[program]
pub mod vote_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;
        vote_account.candidates = Vec::new();
        Ok(())
    }

    pub fn add_candidate(ctx: Context<AddCandidate>, nickname: String, wallet_address: Pubkey) -> Result<u64> {
        let vote_account = &mut ctx.accounts.vote_account;
        let candidate = Candidate {
            nickname,
            votes: 0,
            wallet_address,
        };
        vote_account.candidates.push(candidate);
        Ok((vote_account.candidates.len() - 1) as u64)
    }

    pub fn vote(ctx: Context<Vote>, candidate_index: u64) -> Result<()> {
        let vote_account = &mut ctx.accounts.vote_account;
        if (candidate_index as usize) < vote_account.candidates.len() {
            vote_account.candidates[candidate_index as usize].votes += 1;
            Ok(())
        } else {
            Err(ErrorCode::InvalidCandidateIndex.into())
        }
    }

    pub fn view_votes(ctx: Context<ViewVotes>, candidate_index: u64) -> Result<()> {
        let vote_account = &ctx.accounts.vote_account;
        if (candidate_index as usize) < vote_account.candidates.len() {
            let candidate = &vote_account.candidates[candidate_index as usize];
            msg!("Candidate '{}' (address: {}) has {} votes", 
                candidate.nickname, 
                candidate.wallet_address, 
                candidate.votes
            );
            Ok(())
        } else {
            Err(ErrorCode::InvalidCandidateIndex.into())
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 1000)]
    pub vote_account: Account<'info, VoteAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddCandidate<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub vote_account: Account<'info, VoteAccount>,
}

#[derive(Accounts)]
pub struct ViewVotes<'info> {
    pub vote_account: Account<'info, VoteAccount>,
}

#[account]
pub struct VoteAccount {
    pub candidates: Vec<Candidate>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Candidate {
    pub nickname: String,
    pub votes: u64,
    pub wallet_address: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid candidate index")]
    InvalidCandidateIndex,
}