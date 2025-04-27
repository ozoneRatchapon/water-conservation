use anchor_lang::error_code;

#[error_code]
pub enum WaterConservationError {
    InvalidAmount,
    InsufficientBalance,
    InvalidState,
    InvalidInstruction,
    #[msg("Invalid Depin Feed Address")]
    InvalidDepinFeedAddress,
}
