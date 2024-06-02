use super::*;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized caller: `{caller}`!")]
    Unauthorized { caller: Addr },

    #[error("Address {address} is not admin!")]
    NotAdmin { address: Addr },

    #[error("Deposit is {status}!")]
    PermissionDenied { status: Status },

    #[error("Only one type of coin must be passed: `{denom}`!")]
    InvalidCoin { denom: String },
}
