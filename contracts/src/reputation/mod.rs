use anchor_lang::prelude::*;
use crate::ErrorCode;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct UpdateReputation<'info> {
    #[account(
        mut,
        seeds = [b"reputation", user.as_ref()],
        bump,
        constraint = user_reputation.user == user @ ErrorCode::UserMismatch
    )]
    pub user_reputation: Account<'info, UserReputation>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(seeds = [b"authority".as_ref()], bump)]
    pub program_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserReputation {
    pub user: Pubkey,
    pub reputation_score: i64,
    pub reports_submitted: u64,
    pub reports_validated: u64,
    pub last_updated: i64,
}

pub fn update_reputation(ctx: Context<UpdateReputation>, user: Pubkey, change: i64) -> Result<()> {
    let user_reputation = &mut ctx.accounts.user_reputation;
    let authority = &ctx.accounts.authority;
    let program_authority = &ctx.accounts.program_authority;

    // Ensure only the program authority can update reputation
    if authority.key() != program_authority.key() {
        return Err(ErrorCode::Unauthorized.into());
    }

    // Update reputation score
    let new_score = user_reputation.reputation_score.saturating_add(change);
    user_reputation.reputation_score = new_score.clamp(MIN_REPUTATION, MAX_REPUTATION);

    // Update last updated timestamp
    user_reputation.last_updated = Clock::get()?.unix_timestamp;

    // Emit an event for the reputation update
    emit!(ReputationUpdated {
        user,
        old_score: user_reputation.reputation_score,
        new_score,
        change,
    });

    msg!("Updated reputation for user: {}", user);
    msg!("New reputation score: {}", user_reputation.reputation_score);

    Ok(())
}

pub fn get_user_reputation(user: &Pubkey) -> Result<i64> {
    let (user_reputation_pubkey, _) = Pubkey::find_program_address(
        &[b"reputation", user.as_ref()],
        &crate::ID,
    );

    let user_reputation = UserReputation::try_from_account_info(&user_reputation_pubkey.to_account_info())?;
    Ok(user_reputation.reputation_score)
}

pub fn initialize_user_reputation(ctx: Context<InitializeUserReputation>, user: Pubkey) -> Result<()> {
    let user_reputation = &mut ctx.accounts.user_reputation;
    
    user_reputation.user = user;
    user_reputation.reputation_score = INITIAL_REPUTATION;
    user_reputation.reports_submitted = 0;
    user_reputation.reports_validated = 0;
    user_reputation.last_updated = Clock::get()?.unix_timestamp;

    Ok(())
}

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct InitializeUserReputation<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 8 + 8 + 8,
        seeds = [b"reputation", user.as_ref()],
        bump
    )]
    pub user_reputation: Account<'info, UserReputation>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Event emitted when a user's reputation is updated
#[event]
pub struct ReputationUpdated {
    pub user: Pubkey,
    pub old_score: i64,
    pub new_score: i64,
    pub change: i64,
}

// Constants
const MIN_REPUTATION: i64 = -1000;
const MAX_REPUTATION: i64 = 1000;
const INITIAL_REPUTATION: i64 = 0;