#[cfg(not(feature = "library"))]
use super::*;

/// Handle messages that create and interact with with native provenance markers.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        ManageRoles { roles } => try_manage_roles(deps, info, roles),
        Deposit {} => try_deposit(deps, info),
        CloseDeposit {} => try_close_deposit(deps, info.sender),
        Send { player, fees } => try_send(deps, info.sender, env.contract.address, player, fees),
    }
}

/// Function to manage different roles
/// This function can perform batch operations, hence multiple addresses can be added or removed simultaneously.
/// This entry point can be use to modify different roles such as:-
///     - Admins
/// Based on the `update_type` field addresses can be added or removed:-
///     - for addition `update_type` will be `UpdateType::Add(Addresses)`
///     - for removal `update_type` will be `UpdateType::Remove(Addresses)`
///
/// Arguements:-
///     - List of Roles, can be Admins only at the moment
///     - UpdateType, can be either Add or Remove
///     - List of addresses
///
/// Fails when:-
///     - caller is not admin,
///     - ADMINS map is empty in case of admin removals
///
/// Based on operation, any event can be emitted:-
///     - provwasm.contracts.yolo.add_admins
///     - provwasm.contracts.yolo.remove_admins
pub fn try_manage_roles(
    deps: DepsMut,
    info: MessageInfo,
    roles: Vec<Role>,
) -> Result<Response, ContractError> {
    let mut attrs = Vec::new();
    // Ensuring caller has the admin rights
    is_admin(&deps, info.sender.clone())?;

    for role in roles {
        match role {
            Role::Admins { update_type } => match update_type {
                UpdateType::Add(addrs) => {
                    if ADMINS
                        .update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                            addresses.extend(addrs.clone());
                            addresses.sort();
                            addresses.dedup();
                            Ok(addresses)
                        })
                        .is_err()
                    {
                        ADMINS.save(deps.storage, &addrs)?;
                    };
                    attrs.push(attr("action", "provwasm.contracts.yolo.add_admins"));
                }
                UpdateType::Remove(addrs) => {
                    ADMINS.update(deps.storage, |mut addresses| -> Result<_, ContractError> {
                        addresses.retain(|addr| !addrs.contains(addr));
                        Ok(addresses)
                    })?;
                    attrs.push(attr("action", "provwasm.contracts.yolo.remove_admins"));
                }
            },
        }
    }

    Ok(Response::new().add_attributes(attrs))
}

/// Function to close deposit
/// Anyone can call this function
///
/// Fails when:-
///     - caller is not having admin rights
///     - deposit status is other than open
///
/// Emits Event:-
///     - provwasm.contracts.yolo.close_deposit
pub fn try_close_deposit(deps: DepsMut, sender: Addr) -> Result<Response, ContractError> {
    // Ensure valid caller
    is_admin(&deps, sender)?;

    SCORE_BOARD.update(deps.storage, |mut scores| -> Result<_, ContractError> {
        // Ensure deposit status
        ensure!(
            scores.status.eq(&Status::Open),
            ContractError::PermissionDenied {
                status: scores.status
            }
        );

        scores.update_status(Status::Closed);

        Ok(scores)
    })?;

    Ok(Response::new().add_attribute("action", "provwasm.contracts.yolo.close_deposit"))
}

/// Function to send reward
///
/// Arguements:-
///     - player address
///     - fees
///
/// Fails when:-
///     - caller is not having admin rights
///     - deposit status is other than Closed or Redeemed
///
/// Emits Event:-
///     - provwasm.contracts.yolo.send
pub fn try_send(
    deps: DepsMut,
    sender: Addr,
    contract_address: Addr,
    player: Addr,
    fees: Uint128,
) -> Result<Response, ContractError> {
    // Ensure valid caller
    is_admin(&deps, sender)?;

    let denom = DENOM.load(deps.storage)?;

    SCORE_BOARD.update(deps.storage, |mut scores| -> Result<_, ContractError> {
        // Ensure deposit status
        ensure!(
            scores.status.eq(&Status::Closed) || scores.status.eq(&Status::Redeemed),
            ContractError::PermissionDenied {
                status: scores.status
            }
        );

        scores.update_status(Status::Redeemed);

        Ok(scores)
    })?;

    let contract_coin = deps.querier.query_balance(contract_address, denom)?;

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(BankMsg::Send {
            to_address: player.to_string(),
            amount: vec![coin(
                contract_coin.amount.u128() - fees.u128(),
                contract_coin.denom,
            )],
        }))
        .add_attribute("action", "provwasm.contracts.yolo.send"))
}

/// Function to deposit crypto to contract
/// Player address and amount will be fetched from environmental variables
///
/// Fails when:-
///     - deposit status is other than open
///     - coin passed is different than expected
///
/// Emits event:-
///     - provwasm.contracts.yolo.cast_vote
pub fn try_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let denom = DENOM.load(deps.storage)?;
    let coin = info.funds[0].clone();

    // Ensure valid coin
    ensure!(coin.denom.eq(&denom), ContractError::InvalidCoin { denom });

    SCORE_BOARD.update(deps.storage, |mut scores| -> Result<_, ContractError> {
        // Ensure deposit status
        ensure!(
            scores.status.eq(&Status::Open),
            ContractError::PermissionDenied {
                status: scores.status
            }
        );

        scores.add(info.sender, info.funds[0].amount);
        Ok(scores)
    })?;

    Ok(Response::new().add_attribute("action", "provwasm.contracts.yolo.deposit"))
}
