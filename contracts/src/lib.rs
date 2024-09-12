use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

mod submission;
mod voting;
mod rewards;
mod reputation;
mod escalation;

use submission::*;
use voting::*;
use rewards::*;
use reputation::*;
use escalation::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod civicaid_dao {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, bump: u8) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.bump = bump;
        state.report_count = 0;
        state.total_rewards_distributed = 0;
        Ok(())
    }

    pub fn submit_report(
        ctx: Context<SubmitReport>,
        description: String,
        location: String,
        media_hash: String,
        category: ReportCategory,
    ) -> Result<()> {
        submission::submit_report(ctx, description, location, media_hash, category)
    }

    pub fn vote_on_report(
        ctx: Context<VoteOnReport>,
        report_id: u64,
        vote_type: VoteType,
    ) -> Result<()> {
        voting::vote_on_report(ctx, report_id, vote_type)
    }

    pub fn distribute_rewards(
        ctx: Context<DistributeRewards>,
        report_id: u64,
        amount: u64,
    ) -> Result<()> {
        rewards::distribute_rewards(ctx, report_id, amount)
    }

    pub fn update_reputation(
        ctx: Context<UpdateReputation>,
        user: Pubkey,
        change: i64,
    ) -> Result<()> {
        reputation::update_reputation(ctx, user, change)
    }

    pub fn escalate_report(
        ctx: Context<EscalateReport>,
        report_id: u64,
        reason: String,
    ) -> Result<()> {
        escalation::escalate_report(ctx, report_id, reason)
    }

    pub fn update_report_status(
        ctx: Context<UpdateReportStatus>,
        report_id: u64,
        new_status: ReportStatus,
    ) -> Result<()> {
        submission::update_report_status(ctx, report_id, new_status)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 1 + 8 + 8)]
    pub state: Account<'info, ProgramState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProgramState {
    pub authority: Pubkey,
    pub bump: u8,
    pub report_count: u64,
    pub total_rewards_distributed: u64,
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

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid report status transition")]
    InvalidStatusTransition,
    #[msg("Report not found")]
    ReportNotFound,
    #[msg("Insufficient funds for reward distribution")]
    InsufficientFunds,
    #[msg("User not authorized")]
    Unauthorized,
}