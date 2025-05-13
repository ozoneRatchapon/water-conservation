use anchor_lang::error_code;

#[error_code]
pub enum GreenmoveError {
    InvalidAmount,
    InsufficientBalance,
    InvalidState,
    InvalidInstruction,
    #[msg("Invalid Depin Feed Address")]
    InvalidDepinFeedAddress,
    #[msg("Invalid Property Account")]
    InvalidPropertyAccount,
    #[msg("Invalid Timestamp")]
    InvalidTimestamp,
    #[msg("Timestamps are out of order")]
    TimestampsOutOfOrder,
    #[msg("Excessive energy consumption")]
    ExcessiveEnergyConsumption,
    #[msg("Excessive water consumption")]
    ExcessiveWaterConsumption,
    #[msg("Insufficient points")]
    InsufficientPoints,
    #[msg("Invalid usage data")]
    InvalidUsageData,
    #[msg("Invalid energy consumption data")]
    InvalidEnergyConsumptionData,
    #[msg("Invalid property external ID")]
    InvalidPropertyExternalId,
    #[msg("Invalid water external ID")]
    InvalidWaterExternalId,
    #[msg("Invalid energy external ID")]
    InvalidEnergyExternalId,
}
