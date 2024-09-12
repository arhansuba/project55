use anchor_lang::prelude::*;
use crate::submission::{Report, ReportStatus};
use crate::ErrorCode;

#[derive(Accounts)]
pub struct EscalateReport<'info> {
    #[account(mut)]
    pub report: Account<'info, Report>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [b"authority".as_ref()], bump)]
    pub program_authority: AccountInfo<'info>,
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
}

pub fn escalate_report(ctx: Context<EscalateReport>, report_id: u64, reason: EscalationReason, description: String) -> Result<()> {
    let report = &mut ctx.accounts.report;
    let user = &ctx.accounts.user;

    // Ensure the report exists and is in a state that can be escalated
    if report.key() != report_id {
        return Err(ErrorCode::ReportNotFound.into());
    }

    if report.status != ReportStatus::Submitted && report.status != ReportStatus::UnderReview {
        return Err(ErrorCode::InvalidStatusTransition.into());
    }

    // Check if the user has sufficient reputation to escalate
    // This is a placeholder - you should implement a proper reputation check
    if !has_sufficient_reputation(user.key()) {
        return Err(ErrorCode::Unauthorized.into());
    }

    // Create escalation details
    let escalation_details = EscalationDetails {
        reason,
        description,
        escalated_at: Clock::get()?.unix_timestamp,
        escalated_by: *user.key,
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

// Placeholder function - implement actual reputation checking logic
fn has_sufficient_reputation(_user: &Pubkey) -> bool {
    // TODO: Implement reputation checking logic
    true
}

// Event emitted when a report is escalated
#[event]
pub struct ReportEscalated {
    pub report_id: u64,
    pub escalated_by: Pubkey,
    pub reason: EscalationReason,
}