use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vesting {
    pub root: [u8; 32],
    pub start_ts: u64,
    pub end_ts: u64,
}

#[account]
pub struct UserClaim {
    pub user: Pubkey,
    pub total_allocation: u64,
    pub claimed_amount: u64,
}