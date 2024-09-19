use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Ownership error: {0}")]
    Ownership(#[from] cw_ownable::OwnershipError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Invalid funds. {reason}")]
    InvalidFunds { reason: String },

    #[error("Insufficient wait. You can claim again in {wait_time} seconds")]
    InsufficientWait { wait_time: u64 },
}
