use crate::contract::{execute, instantiate, query};
use crate::tests::mock_env_block_time;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, Decimal, StdError, Uint128,to_binary};
use services::staking::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, StakerInfoResponse,Cw20HookMsg,StakerLockedInfoResponse,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
       // ownership: "owner0000".to_string(),
        staking_token: "staking0000".to_string(),
    };

    let info = mock_info("addr0000", &[]);

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // it worked, let's query the state
    let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let config: ConfigResponse = from_binary(&res).unwrap();
    assert_eq!(
        config,
        ConfigResponse {
            ownership: "addr0000".to_string(),
            staking_token: "staking0000".to_string(),
        }
    );

    

    // let res = query(
    //     deps.as_ref(),
    //     mock_env(),
    //     QueryMsg::State { time_seconds: None },
    // )
    // .unwrap();
    // let state: StateResponse = from_binary(&res).unwrap();
    // assert_eq!(
    //     state,
    //     StateResponse {
    //         last_distributed: mock_env_block_time(),
    //         total_bond_amount: Uint128::zero(),
    //         global_reward_index: Decimal::zero(),
    //     }
    // );
}
#[test]
pub fn receive_cw20()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::new(200000000000),
        msg: to_binary(&Cw20HookMsg::Bond {}).unwrap(),
    });
        let info = mock_info("staking0000", &[]);
    let mut env = mock_env();
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        assert_eq!(
        from_binary::<StakerInfoResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::StakerInfo
                {
                    staker_address:"addr0000".to_string(),
                    // stake_amount:Uint128::new(20000000000),
                    // start_time:env.block.time.seconds(),
                    // fee:Uint128::new(1),
                    // tire:Uint128::zero(),
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(198600000000),
            start_time:env.block.time.seconds(),
            fee:Uint128::new(1400000000),
            tire:Uint128::zero(),
        }
    );
}

#[test]
pub fn receive_cw20_locked()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::new(200000000000),
        msg: to_binary(&Cw20HookMsg::Locked {month:1}).unwrap(),
    });
        let info = mock_info("staking0000", &[]);
    let mut env = mock_env();
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        assert_eq!(
        from_binary::<StakerLockedInfoResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::StakerLockedInfo
                {
                    staker_address:"addr0000".to_string(),
                    // stake_amount:Uint128::new(20000000000),
                    // start_time:env.block.time.seconds(),
                    // fee:Uint128::new(1),
                    // tire:Uint128::zero(),
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerLockedInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(198600000000),
            start_time:env.block.time.seconds(),
            fee:Uint128::new(1400000000),
            tire:Uint128::zero(),
            month:1,
            lock_end:env.block.time.seconds()+400,
        }
    );


}


#[test]
pub fn withdraw_Unlocked_bonus()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Withdraw{};
        let info = mock_info("staking0000", &[]);
    let mut env = mock_env();
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        assert_eq!(
        from_binary::<StakerInfoResponse>(
            &query(
                deps.as_ref(),
                mock_env(),
                QueryMsg::StakerInfo
                {
                    staker_address:"addr0000".to_string(),
                    // stake_amount:Uint128::new(20000000000),
                    // start_time:env.block.time.seconds(),
                    // fee:Uint128::new(1),
                    // tire:Uint128::zero(),
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::zero(),
            start_time:0,
            fee:Uint128::zero(),
            tire:Uint128::zero(),
          //  month:0,
        }
    );


}

#[test]
pub fn receive_cw20_unauthorized()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::new(200000000000),
        msg: to_binary(&Cw20HookMsg::Bond {}).unwrap(),
    });
        let info = mock_info("staking0001", &[]);
    let mut env = mock_env();
    let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
                Err(StdError::GenericErr { msg, .. }) => assert_eq!(msg, "unauthorized"),
                _ => panic!("Must return unauthorized error"),
            }

        
}


#[test]
pub fn receive_cw20_failed_zero_amount()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::zero(),
        msg: to_binary(&Cw20HookMsg::Bond {}).unwrap(),
    });
        let info = mock_info("staking0000", &[]);
    let mut env = mock_env();
    let res = execute(deps.as_mut(), mock_env(), info, msg);
    match res {
        Err(StdError::GenericErr { msg, .. }) => assert_eq!(msg, "amount is zero"),
        _ => panic!("amount zero cant withdraw"),
    }
     
}



// #[test]
// fn change_owner() {
//     let mut deps = mock_dependencies(&[]);
//     let owner = "owner0000".to_string();
//     let new_owner = "owner0001".to_string();

//     let msg = InstantiateMsg {
//         owner: owner.clone(),
//         psi_token: "psi_token".to_string(),
//         staking_token: "staking0000".to_string(),
//         terraswap_factory: "terraswap_factory0000".to_string(),
//         distribution_schedule: vec![StakingSchedule::new(100, 110, Uint128::from(1000000u128))],
//     };

//     let info = mock_info("addr0000", &[]);
//     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//     let msg = ExecuteMsg::UpdateOwner {
//         owner: new_owner.clone(),
//     };
//     let info = mock_info(&owner, &[]);
//     let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();

//     assert_eq!(
//         from_binary::<ConfigResponse>(
//             &query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap()
//         )
//         .unwrap(),
//         ConfigResponse {
//             owner: new_owner.clone(),
//             psi_token: "psi_token".to_string(),
//             staking_token: "staking0000".to_string(),
//             terraswap_factory: "terraswap_factory0000".to_string(),
//             distribution_schedule: vec![StakingSchedule::new(100, 110, Uint128::from(1000000u128))],
//         }
//     );

//     //try to change owner again, but from old owner
//     let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
//     assert!(res.is_err());
//     if let StdError::GenericErr { msg } = res.err().unwrap() {
//         assert_eq!("unauthorized", msg);
//     } else {
//         panic!("wrong error");
//     }
// }
