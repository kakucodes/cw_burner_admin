use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Payment Error: {0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("NotAuthorized: {reason:?}")]
    NotAuthorized { reason: String },

    #[error("ValidationError: {reason:?}")]
    ValidationError { reason: String },

    #[error("Contract is not the token admin ({0})")]
    NotTokenAdmin(String),
}

impl From<ContractError> for StdError {
    fn from(err: ContractError) -> Self {
        StdError::generic_err(err.to_string())
    }
}
