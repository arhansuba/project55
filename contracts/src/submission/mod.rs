use anchor_lang::prelude::*;
use crate::reputation::UserReputation;
use crate::ErrorCode;

#[derive(Accounts)]
pub struct SubmitReport<'info> {
    #[account(mut)]
    pub submitter: Signer<'info>,
    #[account(
        init,
        payer = submitter,
        space = 8 + 32 + 256 + 64 + 256 + 8 + 8 + 1 + 32 + 8
    )]
    pub report: Account<'info, Report>,
    #[account(
        mut,
        seeds = [b"reputation", submitter.key().as_ref()],
        bump
    )]
    pub submitter_reputation: Account<'info, UserReputation>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Report {
    pub submitter: Pubkey,
    pub description: String,
    pub location: String,
    pub media_hash: String,
    pub timestamp: i64,
    pub votes: i64,
    pub category: ReportCategory,
    pub status: ReportStatus,
    pub escalation: Option<Pubkey>,
    pub reward_distributed: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ReportCategory {
    RoadIssue,
    StreetLight,
    PublicFacility,
    EnvironmentalConcern,
    Other,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ReportStatus {
    Submitted,
    UnderReview,
    Approved,
    Resolved,
    Rejected,
}

pub fn submit_report(
    ctx: Context<SubmitReport>,
    description: String,
    location: String,
    media_hash: String,
    category: ReportCategory,
) -> Result<()> {
    let report = &mut ctx.accounts.report;
    let submitter = &ctx.accounts.submitter;
    let submitter_reputation = &mut ctx.accounts.submitter_reputation;

    // Validate input
    if description.len() > 256 || location.len() > 64 || media_hash.len() > 256 {
        return Err(ErrorCode::InputTooLong.into());
    }

    // Check if the submitter has sufficient reputation to submit a report
    if submitter_reputation.reputation_score < MINIMUM_REPUTATION_FOR_SUBMISSION {
        return Err(ErrorCode::InsufficientReputation.into());
    }

    report.submitter = *submitter.key;
    report.description = description;
    report.location = location;
    report.media_hash = media_hash;
    report.timestamp = Clock::get()?.unix_timestamp;
    report.votes = 0;
    report.category = category;
    report.status = ReportStatus::Submitted;
    report.escalation = None;
    report.reward_distributed = false;

    // Update submitter's reputation
    submitter_reputation.reports_submitted += 1;

    // Emit an event for the new report submission
    emit!(ReportSubmitted {
        report_pubkey: report.key(),
        submitter: *submitter.key,
        category: report.category.clone(),
    });

    msg!("Report submitted by: {}", submitter.key);

    Ok(())
}

// Event emitted when a new report is submitted
#[event]
pub struct ReportSubmitted {
    pub report_pubkey: Pubkey,
    pub submitter: Pubkey,
    pub category: ReportCategory,
}

// Constants
const MINIMUM_REPUTATION_FOR_SUBMISSION: i64 = -500; // Adjust this value as needed

// Add this to your lib.rs or main program file
pub fn update_report_status(ctx: Context<UpdateReportStatus>, new_status: ReportStatus) -> Result<()> {
    let report = &mut ctx.accounts.report;
    let authority = &ctx.accounts.authority;

    // Ensure only the program authority can update report status
    if authority.key() != ctx.accounts.program_authority.key() {
        return Err(ErrorCode::Unauthorized.into());
    }

    // Update report status
    report.status = new_status;

    // Emit an event for the status update
    emit!(ReportStatusUpdated {
        report_pubkey: report.key(),
        old_status: report.status.clone(),
        new_status,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateReportStatus<'info> {
    #[account(mut)]
    pub report: Account<'info, Report>,
    pub authority: Signer<'info>,
    #[account(seeds = [b"authority".as_ref()], bump)]
    pub program_authority: AccountInfo<'info>,
}

// Event emitted when a report's status is updated
#[event]
pub struct ReportStatusUpdated {
    pub report_pubkey: Pubkey,
    pub old_status: ReportStatus,
    pub new_status: ReportStatus,
}