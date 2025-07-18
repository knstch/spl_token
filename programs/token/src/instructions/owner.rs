use anchor_lang::prelude::*;
use crate::state::owner_state::TokenOwner;

#[derive(Accounts)]
pub struct InitOwner<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init,
    payer = admin,
    space = 8 + TokenOwner::INIT_SPACE,
    seeds = [b"owner"],
    bump,
    )]
    pub owner: Account<'info, TokenOwner>,
    pub system_program: Program<'info, System>,
}

pub fn init_owner(ctx: Context<InitOwner>) -> Result<()> {
    let state = &mut ctx.accounts.owner;
    state.owner = ctx.accounts.admin.key();

    Ok(())
}