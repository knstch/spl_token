use anchor_lang::Accounts;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount, mint_to, set_authority, SetAuthority};
use anchor_spl::token::spl_token::instruction::AuthorityType;

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
    mut,
    seeds = [b"mint"],
    bump
    )]
    pub mint: Account<'info, Mint>,

    #[account(
    init_if_needed,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = mint)]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

const TOTAL_SUPPLY: u64 = 100_000_000;

pub fn mint_supply(ctx: Context<MintTokens>) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[b"mint", &[ctx.bumps.mint]]];

    mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.mint.to_account_info(),
            }
        ).with_signer(signer_seeds),
        TOTAL_SUPPLY * 10u64.pow(ctx.accounts.mint.decimals as u32),
    )?;

    set_authority(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SetAuthority{
            current_authority: ctx.accounts.mint.to_account_info(),
            account_or_mint: ctx.accounts.mint.to_account_info(),
        }
    ).with_signer(signer_seeds),
    AuthorityType::MintTokens, None
    )?;

    Ok(())
}