use anchor_lang::prelude::*;

#[error_code]
pub enum TokenError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid proof")]
    InvalidProof,
    #[msg("Invalid time")]
    InvalidTime,
    #[msg("Invalid allocation")]
    InvalidAllocation,
}