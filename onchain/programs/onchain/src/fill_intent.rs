use anchor_lang::prelude::*;

// use crate::intent;

#[derive(Accounts)]
pub struct FillIntent<'info> {
    #[account(mut)]
    pub solver: Signer<'info>,
}