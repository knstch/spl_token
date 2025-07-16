#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod instructions;

use {
    anchor_lang::prelude::*,

    instructions::*,
};

declare_id!("F5Law78kGmxJ4igLNhFmpnSGgaZiTjkkxZBnr31VjEeH");

#[program]
pub mod token {
    use super::*;

    pub fn create_token(ctx: Context<CreateToken>,
                        name: String,
                        symbol: String,
                        uri: String) -> Result<()> {
        create::create_token(ctx, name, symbol, uri)
    }
    
    pub fn mint_supply(ctx: Context<MintTokens>) -> Result<()> {
        mint::mint_supply(ctx)
    }
}
