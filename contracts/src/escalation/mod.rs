use anchor_lang::prelude::*;
use crate::submission::{Report, ReportStatus};
use crate::reputation::get_user_reputation;
use crate::ErrorCode;

#[derive(Accounts)]
#[instruction(report_id: u64)]
pub struct EscalateReport<'info> {
    #[account(
        mut,
        constraint = report.key() == report_id @ ErrorCode::ReportNotFound,
        constraint = report.status == ReportStatus::Submitted || report.status == ReportStatus::UnderReview @ ErrorCode::InvalidStatusTransition
    )]
    pub report: Account<'info, Report>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [b"authority".as_ref()], bump)]
    pub program_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EscalationReason {
    Urgent,
    HighImpact,
    PublicSafety,
    RecurringIssue,
    Other,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EscalationDetails {
    pub reason: EscalationReason,
    pub description: String,
    pub escalated_at: i64,
    pub escalated_by: Pubkey,
    pub resolved: bool,
    pub resolution_details: Option<String>,
}

pub fn escalate_report(ctx: Context<EscalateReport>, report_id: u64, reason: EscalationReason, description: String) -> Result<()> {
    let report = &mut ctx.accounts.report;
    let user = &ctx.accounts.user;

    // Check if the user has sufficient reputation to escalate
    let user_reputation = get_user_reputation(user.key())?;
    if user_reputation < MINIMUM_REPUTATION_FOR_ESCALATION {
        return Err(ErrorCode::InsufficientReputation.into());
    }

    // Create escalation details
    let escalation_details = EscalationDetails {
        reason,
        description,
        escalated_at: Clock::get()?.unix_timestamp,
        escalated_by: *user.key,
        resolved: false,
        resolution_details: None,
    };

    // Update report status and add escalation details
    report.status = ReportStatus::UnderReview;
    report.escalation = Some(escalation_details);

    // Emit an event for the escalation
    emit!(ReportEscalated {
        report_id,
        escalated_by: *user.key,
        reason: reason.clone(),
    });

    Ok(())
}

pub fn resolve_escalation(ctx: Context<ResolveEscalation>, report_id: u64, resolution_details: String) -> Result<()> {
    let report = &mut ctx.accounts.report;
    let authority = &ctx.accounts.program_authority;

    // Ensure only the program authority can resolve escalations
    if !authority.is_signer {
        return Err(ErrorCode::Unauthorized.into());
    }

    // Update escalation details
    if let Some(escalation) = &mut report.escalation {
        escalation.resolved = true;
        escalation.resolution_details = Some(resolution_details);
    } else {
        return Err(ErrorCode::NoEscalationFound.into());
    }

    // Update report status
    report.status = ReportStatus::Resolved;

    // Emit an event for the resolution
    emit!(EscalationResolved {
        report_id,
        resolved_by: *authority.key,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(report_id: u64)]
pub struct ResolveEscalation<'info> {
    #[account(
        mut,
        constraint = report.key() == report_id @ ErrorCode::ReportNotFound,
        constraint = report.status == ReportStatus::UnderReview @ ErrorCode::InvalidStatusTransition
    )]
    pub report: Account<'info, Report>,
    #[account(seeds = [b"authority".as_ref()], bump)]
    pub program_authority: Signer<'info>,
}

// Event emitted when a report is escalated
#[event]
pub struct ReportEscalated {
    pub report_id: u64,
    pub escalated_by: Pubkey,
    pub reason: EscalationReason,
}

// Event emitted when an escalation is resolved
#[event]
pub struct EscalationResolved {
    pub report_id: u64,
    pub resolved_by: Pubkey,
}

// Constants
const MINIMUM_REPUTATION_FOR_ESCALATION: i64 = 100; // Adjust this value as needed