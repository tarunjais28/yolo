use super::*;
use crate::contract::*;
use cosmwasm_std::{
    from_json,
    testing::{mock_env, mock_info, MockApi},
    Coin, MemoryStorage, OwnedDeps,
};
use provwasm_mocks::{mock_provenance_dependencies, MockProvenanceQuerier};
const DENOM: &str = "nhash";

fn do_init(
    deps: &mut OwnedDeps<MemoryStorage, MockApi, MockProvenanceQuerier>,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    instantiate(
        deps.as_mut(),
        env,
        info,
        InitMsg {
            denom: DENOM.to_string(),
        },
    )
}

fn do_deposits(deps: &mut OwnedDeps<MemoryStorage, MockApi, MockProvenanceQuerier>, env: Env) {
    // Deposit by player 1
    let mut player = Addr::unchecked("player1");
    let mut amount = Uint128::from(100u128);
    let mut info = mock_info(
        player.as_str(),
        &[Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    );

    // Init contract
    do_init(deps, env.clone(), info.clone()).unwrap();

    let exec_msg = ExecuteMsg::Deposit {};

    execute(deps.as_mut(), env.clone(), info, exec_msg.clone()).unwrap();

    // Deposit by player 2
    player = Addr::unchecked("player2");
    amount = Uint128::from(50u128);
    info = mock_info(
        player.as_str(),
        &[Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    );

    execute(deps.as_mut(), env.clone(), info, exec_msg.clone()).unwrap();

    // Deposit by player 3
    player = Addr::unchecked("player3");
    amount = Uint128::from(150u128);
    info = mock_info(
        player.as_str(),
        &[Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    );

    execute(deps.as_mut(), env.clone(), info, exec_msg.clone()).unwrap();

    // Deposit by player 1 again
    player = Addr::unchecked("player1");
    amount = Uint128::from(100u128);
    info = mock_info(
        player.as_str(),
        &[Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    );

    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();
}

#[test]
fn test_init() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[]);

    let res = do_init(&mut deps, env.clone(), info.clone()).unwrap();
    assert_eq!(0, res.messages.len());

    let expected_score_board = ScoreBoard::default();

    let msg = QueryMsg::GetScoreBoard {};
    let res = query(deps.as_ref(), env, msg).unwrap();
    let score: ScoreBoardView = from_json(&res).unwrap();

    assert_eq!(expected_score_board.convert_to_view(), score);
}

#[test]
fn test_deposit() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Deposit by player 1
    let mut player = Addr::unchecked("player1");
    let mut amount = Uint128::from(100u128);
    let mut info = mock_info(
        player.as_str(),
        &[Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    );

    // Init contract
    do_init(&mut deps, env.clone(), info.clone()).unwrap();

    let exec_msg = ExecuteMsg::Deposit {};

    let res = execute(deps.as_mut(), env.clone(), info, exec_msg.clone()).unwrap();
    assert_eq!(0, res.messages.len());

    let mut expected_score_board = ScoreBoard::default();
    expected_score_board.add(player, amount);

    let qr_msg = QueryMsg::GetScoreBoard {};
    let res = query(deps.as_ref(), env.clone(), qr_msg.clone()).unwrap();
    let score: ScoreBoardView = from_json(&res).unwrap();

    assert_eq!(expected_score_board.convert_to_view(), score);

    // Deposit by player 2
    player = Addr::unchecked("player2");
    amount = Uint128::from(50u128);
    info = mock_info(
        player.as_str(),
        &[Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    );

    let res = execute(deps.as_mut(), env.clone(), info, exec_msg.clone()).unwrap();
    assert_eq!(0, res.messages.len());

    expected_score_board.add(player, amount);

    let res = query(deps.as_ref(), env.clone(), qr_msg.clone()).unwrap();
    let score: ScoreBoardView = from_json(&res).unwrap();

    assert_eq!(expected_score_board.convert_to_view(), score);

    // Deposit by player 3
    player = Addr::unchecked("player3");
    amount = Uint128::from(150u128);
    info = mock_info(
        player.as_str(),
        &[Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    );

    let res = execute(deps.as_mut(), env.clone(), info, exec_msg.clone()).unwrap();
    assert_eq!(0, res.messages.len());

    expected_score_board.add(player, amount);

    let res = query(deps.as_ref(), env.clone(), qr_msg.clone()).unwrap();
    let score: ScoreBoardView = from_json(&res).unwrap();

    assert_eq!(expected_score_board.convert_to_view(), score);

    // Deposit by player 1 again
    player = Addr::unchecked("player1");
    amount = Uint128::from(100u128);
    info = mock_info(
        player.as_str(),
        &[Coin {
            denom: DENOM.to_string(),
            amount,
        }],
    );

    let res = execute(deps.as_mut(), env.clone(), info, exec_msg).unwrap();
    assert_eq!(0, res.messages.len());

    expected_score_board.add(player, amount);

    let res = query(deps.as_ref(), env.clone(), qr_msg).unwrap();
    let score: ScoreBoardView = from_json(&res).unwrap();

    assert_eq!(expected_score_board.convert_to_view(), score);
}

#[test]
fn test_close_deposit() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    let sender = Addr::unchecked("sender");
    let info = mock_info(sender.as_str(), &[]);

    // Init contract
    do_init(&mut deps, env.clone(), info.clone()).unwrap();

    let exec_msg = ExecuteMsg::CloseDeposit {};

    let res = execute(deps.as_mut(), env.clone(), info, exec_msg.clone()).unwrap();
    assert_eq!(0, res.messages.len());

    let mut expected_score_board = ScoreBoard::default();
    expected_score_board.update_status(Status::Closed);

    let qr_msg = QueryMsg::GetScoreBoard {};
    let res = query(deps.as_ref(), env.clone(), qr_msg.clone()).unwrap();
    let score: ScoreBoardView = from_json(&res).unwrap();

    assert_eq!(expected_score_board.convert_to_view(), score);
}

#[test]
fn test_deposit_after_close() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    let sender = Addr::unchecked("sender");
    let info = mock_info(
        sender.as_str(),
        &[Coin {
            denom: DENOM.to_string(),
            amount: Uint128::from(100u128),
        }],
    );

    // Init contract
    do_init(&mut deps, env.clone(), info.clone()).unwrap();

    // Closing Deposit
    let mut exec_msg = ExecuteMsg::CloseDeposit {};
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg.clone()).unwrap();

    // Deposit
    exec_msg = ExecuteMsg::Deposit {};
    let err = execute(deps.as_mut(), env.clone(), info, exec_msg.clone()).unwrap_err();
    assert_eq!(
        ContractError::PermissionDenied {
            status: Status::Closed
        },
        err
    );
}

#[test]
fn test_send_before_close() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("player1");
    let info = mock_info(sender.as_str(), &[]);

    do_deposits(&mut deps, env.clone());

    let exec_msg = ExecuteMsg::Send {
        player: sender,
        fees: Uint128::from(50u128),
    };

    let err = execute(deps.as_mut(), env.clone(), info, exec_msg.clone()).unwrap_err();
    assert_eq!(
        ContractError::PermissionDenied {
            status: Status::Open
        },
        err
    );
}

#[test]
fn test_send() {
    // Create default provenance mocks.
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();
    let sender = Addr::unchecked("player1");
    let info = mock_info(sender.as_str(), &[]);

    do_deposits(&mut deps, env.clone());

    // Close deposit
    let mut exec_msg = ExecuteMsg::CloseDeposit {};
    execute(deps.as_mut(), env.clone(), info.clone(), exec_msg).unwrap();

    exec_msg = ExecuteMsg::Send {
        player: sender.clone(),
        fees: Uint128::from(0u128),
    };

    let res = execute(deps.as_mut(), env.clone(), info, exec_msg.clone()).unwrap();
    assert_eq!(res.messages.len(), 1);
}

#[test]
fn test_try_add_admins() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty admin address
    let init_msg = InitMsg {
        denom: DENOM.to_string(),
    };
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a admin address
    let addr = Addr::unchecked("admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        roles: vec![Role::Admins {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the admin address was set correctly in the contract state
    let admin = ADMINS.load(deps.as_ref().storage).unwrap();
    assert!(admin.contains(&addr));
}

#[test]
fn test_try_remove_admins() {
    let mut deps = mock_provenance_dependencies();
    let env = mock_env();

    // Instantiate the contract with an empty admin address
    let init_msg = InitMsg {
        denom: DENOM.to_string(),
    };
    let info = mock_info("admin", &[]);
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Call try_add_subadmin with a admin address
    let addr = Addr::unchecked("admin");
    let info = mock_info("admin", &[]);
    let msg = ExecuteMsg::ManageRoles {
        roles: vec![Role::Admins {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Check that the admin address was set correctly in the contract state
    let admin = ADMINS.load(deps.as_ref().storage).unwrap();
    assert!(admin.contains(&addr));
    assert_eq!(admin.len(), 1);

    // try admin again
    let msg = ExecuteMsg::ManageRoles {
        roles: vec![Role::Admins {
            update_type: UpdateType::Add(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(1, res.attributes.len());
    let admin = ADMINS.load(deps.as_ref().storage).unwrap();
    assert_eq!(admin.len(), 1);

    // remove admin
    let msg = ExecuteMsg::ManageRoles {
        roles: vec![Role::Admins {
            update_type: UpdateType::Remove(vec![addr.clone()]),
        }],
    };
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    assert_eq!(res.messages.len(), 0);

    // verify admin was removed
    let admin = ADMINS.load(deps.as_ref().storage).unwrap();
    assert_eq!(admin.len(), 0);

    // try to remove admin again
    let msg = ExecuteMsg::ManageRoles {
        roles: vec![Role::Admins {
            update_type: UpdateType::Remove(vec![addr.clone()]),
        }],
    };
    let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    assert_eq!(ContractError::NotAdmin { address: addr }, err);
}
