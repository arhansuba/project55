use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SubmitReport<'info> {
    #[account(mut)]
    pub submitter: Signer<'info>,
    #[account(init, payer = submitter, space = 8 + 32 + 256 + 64 + 256 + 8)]
    pub report: Account<'info, Report>,
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
}

pub fn submit_report(ctx: Context<SubmitReport>, description: String, location: String, media_hash: String) -> Result<()> {
    let report = &mut ctx.accounts.report;
    let submitter = &ctx.accounts.submitter;

    report.submitter = *submitter.key;
    report.description = description;
    report.location = location;
    report.media_hash = media_hash;
    report.timestamp = Clock::get()?.unix_timestamp;
    report.votes = 0;

    Ok(())
}