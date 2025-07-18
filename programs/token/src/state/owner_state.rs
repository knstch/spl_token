use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TokenOwner {
    pub owner: Pubkey,
}