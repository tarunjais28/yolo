use super::*;

#[cw_serde]
pub struct InitMsg {
    pub denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    ManageRoles { roles: Vec<Role> },
    Deposit {},
    CloseDeposit {},
    Send { player: Addr, fees: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<Addr>)]
    GetAdmins {},

    #[returns(ScoreBoardView)]
    GetScoreBoard {},

    #[returns(cosmwasm_std::Coin)]
    GetBalance { denom: String, address: Addr },
}

/// Migrate the contract.
#[cw_serde]
pub struct MigrateMsg {}
