use anchor_lang::prelude::*;
use crate::submission::{Report, ReportStatus};
use crate::reputation::UserReputation;
use crate::ErrorCode;

#[derive(Accounts)]
#[instruction(report_id: u64)]
pub struct VoteOnReport<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(
        mut,
        constraint = report.key() == report_id @ ErrorCode::ReportNotFound,
        constraint = report.status == ReportStatus::Submitted @ ErrorCode::InvalidReportStatus
    )]
    pub report: Account<'info, Report>,
    #[account(
        mut,
        seeds = [b"reputation", voter.key().as_ref()],
        bump
    )]
    pub voter_reputation: Account<'info, UserReputation>,
    #[account(mut)]
    pub submitter_reputation: Account<'info, UserReputation>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VoteType {
    Upvote,
    Downvote,
}

#[account]
pub struct Vote {
    pub voter: Pubkey,
    pub report: Pubkey,
    pub vote_type: VoteType,
    pub timestamp: i64,
}

pub fn vote_on_report(ctx: Context<VoteOnReport>, report_id: u64, vote_type: VoteType) -> Result<()> {
    let report = &mut ctx.accounts.report;
    let voter = &ctx.accounts.voter;
    let voter_reputation = &mut ctx.accounts.voter_reputation;
    let submitter_reputation = &mut ctx.accounts.submitter_reputation;

    // Check if the voter has already voted on this report
    if has_voted(voter.key(), report_id)? {
        return Err(ErrorCode::AlreadyVoted.into());
    }

    // Update vote count
    match vote_type {
        VoteType::Upvote => report.votes += 1,
        VoteType::Downvote => report.votes -= 1,
    }

    // Update reputations
    update_reputations(voter_reputation, submitter_reputation, &vote_type)?;

    // Record the vote
    record_vote(voter.key(), report_id, vote_type)?;

    // Check if the report should be approved or rejected based on votes
    check_report_status(report, submitter_reputation)?;

    // Emit an event for the vote
    emit!(VoteCast {
        report_id,
        voter: *voter.key,
        vote_type: vote_type.clone(),
    });

    msg!("Vote cast by {} on report {}", voter.key, report_id);

    Ok(())
}

fn has_voted(voter: &Pubkey, report_id: u64) -> Result<bool> {
    // TODO: Implement logic to check if the voter has already voted on this report
    // This might involve checking a separate Vote account or a bitmap in the Report account
    Ok(false)
}

fn update_reputations(voter_reputation: &mut Account<UserReputation>, submitter_reputation: &mut Account<UserReputation>, vote_type: &VoteType) -> Result<()> {
    match vote_type {
        VoteType::Upvote => {
            voter_reputation.reputation_score += VOTE_REPUTATION_CHANGE;
            submitter_reputation.reputation_score += UPVOTE_REPUTATION_CHANGE;
        }
        VoteType::Downvote => {
            voter_reputation.reputation_score += VOTE_REPUTATION_CHANGE;
            submitter_reputation.reputation_score -= DOWNVOTE_REPUTATION_CHANGE;
        }
    }
    Ok(())
}

fn record_vote(voter: Pubkey, report_id: u64, vote_type: VoteType) -> Result<()> {
    // TODO: Implement logic to record the vote
    // This might involve creating a new Vote account or updating a bitmap in the Report account
    Ok(())
}

fn check_report_status(report: &mut Account<Report>, submitter_reputation: &mut Account<UserReputation>) -> Result<()> {
    if report.votes >= APPROVAL_THRESHOLD {
        report.status = ReportStatus::Approved;
        submitter_reputation.reputation_score += REPORT_APPROVED_REPUTATION_CHANGE;
    } else if report.votes <= REJECTION_THRESHOLD {
        report.status = ReportStatus::Rejected;
        submitter_reputation.reputation_score -= REPORT_REJECTED_REPUTATION_CHANGE;
    }
    Ok(())
}

// Event emitted when a vote is cast
#[event]
pub struct VoteCast {
    pub report_id: u64,
    pub voter: Pubkey,
    pub vote_type: VoteType,
}

// Constants
const VOTE_REPUTATION_CHANGE: i64 = 1;
const UPVOTE_REPUTATION_CHANGE: i64 = 5;
const DOWNVOTE_REPUTATION_CHANGE: i64 = 2;
const APPROVAL_THRESHOLD: i64 = 5;
const REJECTION_THRESHOLD: i64 = -3;
const REPORT_APPROVED_REPUTATION_CHANGE: i64 = 20;
const REPORT_REJECTED_REPUTATION_CHANGE: i64 = 10;