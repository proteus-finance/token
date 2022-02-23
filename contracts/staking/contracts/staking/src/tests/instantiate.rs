use crate::contract::{execute, instantiate, query};
use crate::tests::{mock_env_block_time};
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
        amount: Uint128::new(2000000),
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
                   
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(1986000),
            start_time:env.block.time.seconds(),
            fee:Uint128::new(14000),
            tire:Uint128::zero(),
            bonus:Uint128::zero(),
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
        amount: Uint128::new(2000000),
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
                   
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerLockedInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(1986000),
            start_time:env.block.time.seconds(),
            fee:Uint128::new(14000),
            tire:Uint128::zero(),
            month:1,
            lock_end:1574389419,
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
    let info = mock_info("addr0000", &[],);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Withdraw{amount_withdraw : Uint128::zero()};
        let info = mock_info("staking0000", &[]);
    let mut env = mock_env();
    let res = execute(deps.as_mut(), mock_env(), info, msg);

    match res {
        Err(StdError::GenericErr { msg, .. }) => assert_eq!(msg, "Please enter correct amount"),
        _ => panic!("Must return unauthorized error"),
    }


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




#[test]
pub fn withdraw_owner()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::WithdrawOwner{amount:Uint128::new(1000000000)};

    let info = mock_info("staking0001", &[]);
    let mut env = mock_env();
    
    let res = execute(deps.as_mut(), mock_env(), info, msg);


    
     
}

#[test]
pub fn withdraw_owner_fail()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::WithdrawOwner{amount:Uint128::new(1000000000)};
    let mut env = mock_env();
    let info = mock_info("addr0001", &[]);
    let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
                Err(StdError::GenericErr { msg, .. }) => assert_eq!(msg, "owner can execute this function"),
                _ => panic!("Must return unauthorized error"),
            }
    

}

#[test]
pub fn withdraw_ussd()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::TransferUsd{amount:Uint128::new(1000000000)};

    let info = mock_info("staking0001", &[]);
    let mut env = mock_env();
    
    let res = execute(deps.as_mut(), mock_env(), info, msg);
    
     
}


#[test]
pub fn withdraw_uluna()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::TransferLuna{amount:Uint128::new(1000000000)};

    let info = mock_info("staking0001", &[]);
    let mut env = mock_env();
    
    let res = execute(deps.as_mut(), mock_env(), info, msg);
    
     
}



#[test]
pub fn withdraw_uluna_fail()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::TransferLuna{amount:Uint128::new(1000000000)};
    let mut env = mock_env();
    let info = mock_info("addr0001", &[]);
    let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
                Err(StdError::GenericErr { msg, .. }) => assert_eq!(msg, "owner can execute this function"),
                _ => panic!("Must return unauthorized error"),
            }
    
     
}


#[test]
pub fn withdraw_ussd_fail()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::TransferUsd{amount:Uint128::new(1000000000)};
    let mut env = mock_env();
    let info = mock_info("addr0001", &[]);
    let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
                Err(StdError::GenericErr { msg, .. }) => assert_eq!(msg, "owner can execute this function"),
                _ => panic!("Must return unauthorized error"),
            }
    
     
}


#[test]
pub fn unlocked_staker_tire1()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::new(20000000000),
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
                 
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(19860000000),
            start_time:env.block.time.seconds(),
            fee:Uint128::new(140000000),
            tire:Uint128::new(1),
            bonus:Uint128::zero(),
        }
    );
}



#[test]
pub fn unlocked_staker_tire2()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::new(99000000000),
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
                    
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(98505000000),
            start_time:env.block.time.seconds(),
            fee:Uint128::new(495000000),
            tire:Uint128::new(2),
            bonus:Uint128::zero(),
        }
    );
}



#[test]
pub fn unlocked_staker_tire3()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::new(100000000000),
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
                
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(99700000000),
            start_time:env.block.time.seconds(),
            fee:Uint128::new(300000000),
            tire:Uint128::new(3),
            bonus:Uint128::zero(),
        }
    );
}

#[test]
pub fn unlocked_staker_tire4()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::new(1999990000000000),
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
                    
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(1999990000000000),
            start_time:env.block.time.seconds(),
            fee:Uint128::zero(),
            tire:Uint128::new(4),
            bonus:Uint128::zero(),
        }
    );
}




#[test]
pub fn locked_staker_tire1_month1()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::new(20000000000),
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
                  
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerLockedInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(19860000000),
            start_time:env.block.time.seconds(),
            fee:Uint128::new(140000000),
            tire:Uint128::new(1),
            month:1,
            lock_end:1574389419,
        }
    );
}



#[test]
pub fn locked_staker_tire2()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::new(99000000000),
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
                   
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerLockedInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(98505000000),
            start_time:env.block.time.seconds(),
            fee:Uint128::new(495000000),
            tire:Uint128::new(2),
            month:1,
            lock_end:1574389419,
        }
    );
}


#[test]
pub fn locked_staker_tire3()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::new(100000000000),
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
                    
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerLockedInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(99700000000),
            start_time:env.block.time.seconds(),
            fee:Uint128::new(300000000),
            tire:Uint128::new(3),
            month:1,
            lock_end:1574389419,
        }
    );
}


#[test]
pub fn locked_staker_tire4()
{
    let mut deps = mock_dependencies(&[]);
    let msg = InstantiateMsg{
    staking_token: "staking0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        sender: "addr0000".to_string(),
        amount: Uint128::new(1999990000000000),
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
                    
                }
            )
            .unwrap()
        )
        .unwrap(),
        StakerLockedInfoResponse {
            staker_address:"addr0000".to_string(),
            stake_amount:Uint128::new(1999990000000000),
            start_time:env.block.time.seconds(),
            fee:Uint128::zero(),
            tire:Uint128::new(4),
            month:1,
            lock_end:1574389419,
        }
    );
}






