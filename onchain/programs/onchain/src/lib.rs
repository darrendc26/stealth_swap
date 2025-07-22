use anchor_lang::prelude::*;

declare_id!("EhHrLoTGxc26Y9EPVCc9QWPdZhnSAuApzQZeipkdTWof");

#[program]
pub mod onchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
