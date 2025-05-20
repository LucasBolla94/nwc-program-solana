use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The contract is currently paused.")]
    ContractPaused,

    #[msg("Math operation overflowed.")]
    MathOverflow,

    #[msg("Liquid is not sufficient now, try later.")]
    InsufficientLiquidity,

    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
