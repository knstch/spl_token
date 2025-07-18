use {
    crate::{
        instructions::errors::TokenError, state::vesting::{
            UserClaim,
            Vesting,
        }
    }, anchor_lang::prelude::*, anchor_spl::{
        associated_token::AssociatedToken,
        token::{self, Mint, Token, TokenAccount, Transfer},
    }
};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(seeds = [b"vesting"], bump)]
    pub vesting: Account<'info, Vesting>,
    #[account(init_if_needed, 
        seeds = [b"user_claim", user.key().as_ref()], 
        bump, 
        payer = user, 
        space = 8 + UserClaim::INIT_SPACE,
    )]
    pub user_claim: Account<'info, UserClaim>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        seeds = [b"mint"],
        bump
        )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = mint,
        )]
    pub treasure: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

pub fn claim(ctx: Context<Claim>, allocation: u64, proof: Vec<[u8; 32]>) -> Result<()> {
    let vesting = &mut ctx.accounts.vesting;
    let user_claim = &mut ctx.accounts.user_claim;

    let mut computed_hash = anchor_lang::solana_program::keccak::hashv(&[
        ctx.accounts.user.key().as_ref(),
        &allocation.to_le_bytes(),
    ]).0;

    for sibling in proof.iter() {
        if computed_hash <= *sibling {
            computed_hash = anchor_lang::solana_program::keccak::hashv(&[&computed_hash, sibling]).0;
        } else {
            computed_hash = anchor_lang::solana_program::keccak::hashv(&[sibling, &computed_hash]).0;
        }
    }

    let time_now = Clock::get()?.unix_timestamp as u64;

    require!(Pubkey::from(computed_hash) == Pubkey::from(vesting.root), TokenError::InvalidProof);
    require!(vesting.start_ts <= time_now, TokenError::InvalidTime);

    if user_claim.total_allocation == 0 {
        user_claim.total_allocation = allocation;
    }

    let elapsed = time_now.saturating_sub(vesting.start_ts);
    let duration = vesting.end_ts.saturating_sub(vesting.start_ts);

    let unlocked_amount = if time_now >= vesting.end_ts {
        user_claim.total_allocation - user_claim.claimed_amount
    } else {
        (user_claim.total_allocation as u128)
            .checked_mul(elapsed as u128)
            .unwrap()
            .checked_div(duration as u128)
            .unwrap() as u64
    };

    require!(unlocked_amount >= allocation, TokenError::InvalidAllocation);

    user_claim.claimed_amount += unlocked_amount;

    let cpi_accounts = Transfer {
        from: ctx.accounts.treasure.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
        authority: ctx.accounts.mint.to_account_info(),
    };

    let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[ctx.bumps.mint]]];
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        cpi_accounts).with_signer(signer_seeds);

    token::transfer(cpi_ctx, unlocked_amount)?;

    Ok(())
}