#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod instructions;
pub mod state;

use {
    anchor_lang::prelude::*,

    instructions::*,
};

declare_id!("Ecveh3AEh6kjcKfHqSgLfY4KKJ8nZHZxvJW5D8VejPr8");

#[program]
pub mod token {
    use super::*;

    pub fn create_token(ctx: Context<CreateToken>, name: String, symbol: String, uri: String) -> Result<()> {
        create::create_token(ctx, name, symbol, uri)
    }

    pub fn mint_supply(ctx: Context<MintTokens>) -> Result<()> {
        mint::mint_supply(ctx)
    }

    pub fn init_owner(ctx: Context<InitOwner>) -> Result<()> {
        owner::init_owner(ctx)
    }

    pub fn initialize_vesting(ctx: Context<InitializeVesting>, root: [u8; 32], start_ts: u64, end_ts: u64) -> Result<()> {
        vesting::initialize_vesting(ctx, root, start_ts, end_ts)
    }
}
