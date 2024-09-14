use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::submission::Report;
use crate::ErrorCode;

#[derive(Accounts)]
#[instruction(report_id: u64)]
pub struct DistributeRewards<'info> {
    #[account(
        mut,
        constraint = report.key() == report_id @ ErrorCode::ReportNotFound,
        constraint = report.submitter == submitter.key() @ ErrorCode::InvalidSubmitter,
        constraint = !report.rewarded @ ErrorCode::AlreadyRewarded
    )]
    pub report: Account<'info, Report>,
    #[account(mut)]
    pub submitter: Signer<'info>,
    #[account(
        mut,
        constraint = submitter_token_account.owner == submitter.key() @ ErrorCode::InvalidTokenAccount
    )]
    pub submitter_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = reward_vault.mint == submitter_token_account.mint @ ErrorCode::InvalidMint
    )]
    pub reward_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    #[account(seeds = [b"authority".as_ref()], bump)]
    pub program_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn distribute_rewards(ctx: Context<DistributeRewards>, report_id: u64) -> Result<()> {
    let report = &mut ctx.accounts.report;
    let submitter = &ctx.accounts.submitter;
    let submitter_token_account = &ctx.accounts.submitter_token_account;
    let reward_vault = &ctx.accounts.reward_vault;
    let token_program = &ctx.accounts.token_program;
    let program_authority = &ctx.accounts.program_authority;

    // Calculate reward amount based on report quality or fixed amount
    let reward_amount = calculate_reward_amount(report)?;

    // Transfer tokens from reward vault to submitter
    let transfer_instruction = Transfer {
        from: reward_vault.to_account_info(),
        to: submitter_token_account.to_account_info(),
        authority: program_authority.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            transfer_instruction,
            &[&[b"authority".as_ref(), &[ctx.bumps.program_authority]]]
        ),
        reward_amount
    )?;

    // Mark the report as rewarded
    report.rewarded = true;

    // Emit an event for the reward distribution
    emit!(RewardDistributed {
        report_id,
        submitter: *submitter.key,
        amount: reward_amount,
    });

    msg!("Distributed {} tokens to submitter: {}", reward_amount, submitter.key);

    Ok(())
}

fn calculate_reward_amount(report: &Report) -> Result<u64> {
    // TODO: Implement a more sophisticated reward calculation based on report quality, user reputation, etc.
    // For now, we'll use a fixed amount
    Ok(FIXED_REWARD_AMOUNT)
}

// Event emitted when a reward is distributed
#[event]
pub struct RewardDistributed {
    pub report_id: u64,
    pub submitter: Pubkey,
    pub amount: u64,
}

// Constants
const FIXED_REWARD_AMOUNT: u64 = 100; // Adjust this value as needed

// Add this to your lib.rs or main program file
pub fn initialize_reward_vault(ctx: Context<InitializeRewardVault>, amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: ctx.accounts.funder.to_account_info(),
        to: ctx.accounts.reward_vault.to_account_info(),
        authority: ctx.accounts.funder.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeRewardVault<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,
    #[account(
        init,
        payer = funder,
        token::mint = mint,
        token::authority = program_authority,
    )]
    pub reward_vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, token::Mint>,
    #[account(seeds = [b"authority".as_ref()], bump)]
    pub program_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}