use std::fmt::Display;

use super::*;

#[cw_serde]
pub enum UpdateType<T> {
    Add(T),
    Remove(T),
}

#[cw_serde]
pub enum Role {
    Admins { update_type: UpdateType<Vec<Addr>> },
}

#[cw_serde]
pub enum DestUpdateType {
    Chain(String),
    Address(String),
}

#[cw_serde]
pub enum Status {
    Open,
    Closed,
    Redeemed,
}

impl Default for Status {
    fn default() -> Self {
        Self::Open
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "opened"),
            Self::Closed => write!(f, "closed"),
            Self::Redeemed => write!(f, "redeemed"),
        }
    }
}
