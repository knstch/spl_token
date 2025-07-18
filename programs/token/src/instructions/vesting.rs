use {
    crate::{
        state::vesting_merkle_tree::*,
        state::owner_state::TokenOwner,
        instructions::errors::TokenError,
    },
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct InitializeVesting<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, 
    space = 8 + Vesting::INIT_SPACE,
    payer = payer,
    seeds = [b"vesting"],
    bump,
    )]
    pub vesting: Account<'info, Vesting>,
    #[account(seeds = [b"owner"], bump)]
    pub owner: Account<'info, TokenOwner>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_vesting(ctx: Context<InitializeVesting>, root: [u8; 32], start_ts: u64, end_ts: u64) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.payer.key(),
        ctx.accounts.owner.owner.key(),
        TokenError::Unauthorized
    );

    ctx.accounts.vesting.root = root;
    ctx.accounts.vesting.start_ts = start_ts;
    ctx.accounts.vesting.end_ts = end_ts;

    Ok(())
}