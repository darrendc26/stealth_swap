#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
pub mod create_intent;
pub mod fill_intent;
pub mod intent;
use create_intent::*;

declare_id!("EhHrLoTGxc26Y9EPVCc9QWPdZhnSAuApzQZeipkdTWof");

#[program]
pub mod onchain {
    use super::*;

    pub fn create_intent(ctx: Context<CreateIntent>, args: CreateIntentArgs, user_id: u64) -> Result<()> {
        create_intent::handler(ctx, args, user_id)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
