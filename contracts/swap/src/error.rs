use cosmwasm_std::StdError;
use thiserror::Error;

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.

    #[error("Channel not found")]
    ChannelNotFound {},

    #[error("Swap order not found")]
    OrderNotFound {},

    #[error("Only supports channel with ibc version ics20-1, got {version}")]
    InvalidIbcVersion { version: String },

    #[error("Only supports unordered channel")]
    OnlyOrderedChannel {},

    #[error("Token reply result not found")]
    TokenResultNotFound {},

    #[error("Invalid amount")]
    InvalidAmountValue {},

    #[error("Denom not allowed: {denom}")]
    DenomNotAllowed { denom: String },
}
