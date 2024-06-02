#[cfg(not(feature = "library"))]
use super::*;

/// Initialize the smart contract config state.
///
/// Event:-
///     provwasm.contracts.yolo.init
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    // admin info, granting sender the admin role
    let admin = info.sender;
    ADMINS.save(deps.storage, &vec![admin])?;

    SCORE_BOARD.save(deps.storage, &ScoreBoard::default())?;

    DENOM.save(deps.storage, &msg.denom)?;

    // Dispatch messages to the name module handler and emit an event.
    Ok(Response::new().add_attribute("action", "provwasm.contracts.yolo.init"))
}
