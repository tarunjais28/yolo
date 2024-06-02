#[cfg(not(feature = "library"))]
use super::*;

/// Handle query requests for the provenance marker module.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    use QueryMsg::*;

    match msg {
        GetAdmins {} => try_get_admins(deps),
        GetScoreBoard {} => try_get_scare_board(deps),
        GetBalance { denom, address } => try_get_balance(deps, denom, address),
    }
}

/// Query to get balance of an address.
///
/// Returns:-
///     Balance
fn try_get_balance(deps: Deps, denom: String, address: Addr) -> Result<QueryResponse, StdError> {
    let balance = deps.querier.query_balance(address, denom)?;
    to_json_binary(&balance)
}

/// Query to get admin addresses.
///
/// Returns:-
///     List of admin addresses
fn try_get_admins(deps: Deps) -> Result<QueryResponse, StdError> {
    let admins = ADMINS.load(deps.storage)?;
    to_json_binary(&admins)
}

/// Query to get score board.
///
/// Returns:-
///     List of validator addresses
fn try_get_scare_board(deps: Deps) -> Result<QueryResponse, StdError> {
    let scores = SCORE_BOARD.load(deps.storage)?;
    to_json_binary(&scores.convert_to_view())
}
