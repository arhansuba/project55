use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct VoteOnReport<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub report: Account<'info, Report>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum VoteType {
    Upvote,
    Downvote,
}

pub fn vote_on_report(ctx: Context<VoteOnReport>, report_id: u64, vote_type: VoteType) -> Result<()> {
    let report = &mut ctx.accounts.report;

    match vote_type {
        VoteType::Upvote => report.votes += 1,
        VoteType::Downvote => report.votes -= 1,
    }

    // TODO: Implement logic to check if rewards should be distributed
    // For example, if votes reach a certain threshold

    Ok(())
}