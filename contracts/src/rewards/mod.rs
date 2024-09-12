use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub report: Account<'info, Report>,
    #[account(mut)]
    pub submitter: AccountInfo<'info>,
    #[account(mut)]
    pub reward_vault: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

pub fn distribute_rewards(ctx: Context<DistributeRewards>, report_id: u64) -> Result<()> {
    let report = &ctx.accounts.report;
    let submitter = &ctx.accounts.submitter;
    let reward_vault = &ctx.accounts.reward_vault;
    let token_program = &ctx.accounts.token_program;

    // TODO: Implement token transfer logic
    // For example, transfer a fixed amount of tokens from reward_vault to submitter
    // You'll need to use the Solana token program to perform the transfer

    // This is a placeholder for the actual token transfer logic
    msg!("Distributing rewards to submitter: {}", submitter.key);

    Ok(())
}