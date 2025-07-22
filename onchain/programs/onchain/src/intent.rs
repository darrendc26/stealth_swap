use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Intent {
    pub user: Pubkey,
    pub user_id: u64,
    pub input_token: Pubkey,
    pub output_token: Pubkey,
    pub input_amount: u64,
    pub min_receive: u64,
    pub active: bool,
    pub bump: u8,
}