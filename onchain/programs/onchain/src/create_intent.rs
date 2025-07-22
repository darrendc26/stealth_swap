use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, Transfer, TokenAccount};

use crate::intent::Intent;

#[derive(Accounts)]
#[instruction(user_id: u64)]
pub struct CreateIntent<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + Intent::INIT_SPACE,
        seeds = [
            b"intent".as_ref(),
            user.key().as_ref(),
            &user_id.to_le_bytes()
        ],
        bump,
    )]
    pub intent: Account<'info, Intent>,
    #[account(mut,
        constraint = user_token_account.owner == user.key()
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    pub escrow: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateIntentArgs {
    pub user: Pubkey,
    pub input_token: Pubkey,
    pub output_token: Pubkey,
    pub input_amount: u64,
    pub min_receive: u64,
}

pub fn handler(ctx: Context<CreateIntent>, args: CreateIntentArgs, user_id: u64) -> Result<()> {
    ctx.accounts.intent.set_inner(Intent {
        user: args.user,
        user_id,
        input_token: args.input_token,
        output_token: args.output_token,
        input_amount: args.input_amount,
        min_receive: args.min_receive,
        active: true,
        bump: ctx.bumps.intent,
    });

    let ctx_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.escrow.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    
    let cpi_token_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_token_program, ctx_accounts);
    
    transfer(cpi_context, args.input_amount)?;

    Ok(())
}

