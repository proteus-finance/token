/*  
  this is staking smart contract in which owner of proteus token will 
  stake in which we dealing two types of staking one is locked staking 
  in which user wil stake for specific time if specific time not complete 
  then he cant withdraw but in unlocked staking stker can withdraw his amount 
  any time and he cen stake amount many time. so infomation of locked staking and 
  unlocked staking manged seprately
*/
// in start we impoart some rust pkg thats will support our smart contract

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
     from_binary, to_binary, Addr,  Binary, BlockInfo, CanonicalAddr, Coin,
    Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
     Uint128, WasmMsg,CosmosMsg,BankMsg,
};

use services::staking::{
    ConfigResponse, 
    Cw20HookMsg, ExecuteMsg, InstantiateMsg, 
     QueryMsg,
    StakerInfoResponse,
    StakerLockedInfoResponse,
};

use crate::state::{
    read_config,
     read_staker_info, 
     store_config, 
     store_staker_info,
     store_staker_locked_info,
     read_staker_locked_info,
     StakerLockedInfo,
     Config, StakerInfo,
};

use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};

/*
   when we deploy our smart contract then initate function will execute 
   so in start his function we pass one address of our token so in start 
   our token address will be set.


                      "Instantiate Json"

                      {
                       
                        "staking_token":"terra14u5n457t9lyh3qpzdkjxjhwh0dlcm90whgzzzn"

                      }



*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    store_config(
        deps.storage,
        &Config {
             ownership:_info.sender.to_string(),
            staking_token: deps.api.addr_canonicalize(&msg.staking_token)?,
        },
    )?;



    Ok(Response::default())
}
  

/*
basically this execute function help to execute the function 
when we execute any function then we pay the gass fee those 
function we need to execute we define these function inside execute
function */


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
      
       
        ExecuteMsg::Withdraw {amount_withdraw} => withdraw(deps, env, info,amount_withdraw),

        ExecuteMsg:: WithdrawLocked {} => withdraw_locked(deps, env, info),

        ExecuteMsg::WithdrawOwner {amount} => withdraw_owner(deps, env, info,amount),

        ExecuteMsg::TransferUsd { amount } => {
            execute_transfer_usd(deps, env, info, amount, )
        }

        ExecuteMsg::TransferLuna { amount } => {
            execute_transfer_luna(deps, env, info, amount, )
        }
       
    }
}

/*    this is recieve cw 20 function thats purpose to recive
      cw 20  but this function also will tigger other two function
      one is bond thats will use for unlock staking and other locked
      thats purpose is unlocked staking 
*/

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let config: Config = read_config(deps.storage)?;

    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Bond {}) => {
            // only staking token contract can execute this message
            if config.staking_token != deps.api.addr_canonicalize(info.sender.as_str())? {
                return Err(StdError::generic_err("unauthorized"));
            }

            let cw20_sender = deps.api.addr_validate(&cw20_msg.sender)?;
            bond(deps, env, cw20_sender, cw20_msg.amount)
        }

        Ok(Cw20HookMsg::Locked {month})=>
        {
            if config.staking_token != deps.api.addr_canonicalize(info.sender.as_str())? {
                return Err(StdError::generic_err("unauthorized"));
            }

            let cw20_sender = deps.api.addr_validate(&cw20_msg.sender)?;
            locked(deps, env, cw20_sender, cw20_msg.amount,month)
        }
        Err(_) => Err(StdError::generic_err("data should be given")),
    }
}
/*
    this bond function is used for normal staking  
    and tkae two parameter one is address and second one is amount 
    but we executing this recive cw20 so its will take base 64.

                          "bond function json"



                                  {

                                "send": {
                                "msg": "eyJib25kIjp7fX0=",
                               "amount": "15000000000",
                                "contract": "terra10mdmmerj556fcm57xy45a6ga3rm0wu9mcnzlak"
                                 }
                              }


*/


pub fn bond(deps: DepsMut, env: Env, sender_addr: Addr, amount: Uint128) -> StdResult<Response> {
     let current_time = get_time(&env.block);
     let sender_addr_raw: CanonicalAddr = deps.api.addr_canonicalize(sender_addr.as_str())?;
     let mut bonus = Uint128::zero();
        if amount == Uint128::zero()
        {
            return Err(StdError::generic_err("amount is zero"));
        }

    
     let mut staker_info: StakerInfo = read_staker_info(deps.storage, &sender_addr_raw)?;
      if staker_info.stake_amount >  Uint128::zero()
      {
        let timeinvest = current_time - staker_info.start_time;

        if staker_info.tire == Uint128::zero()
        {  
            bonus = staker_info.stake_amount;
    
        }
    
        if staker_info.tire == Uint128::new(1)
        {
         
        
         let total_profit_percentage = Decimal::from_ratio (10 * timeinvest as u128 ,60*60*24*365 as u128);
         let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
         bonus = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
    
        if staker_info.tire == Uint128::new(2)
        {
            
            let total_profit_percentage= Decimal::from_ratio (12 * timeinvest as u128, 60*60*24*365 as  u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            bonus = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
    
        if staker_info.tire == Uint128::new(3)
        {
            
            let total_profit_percentage= Decimal::from_ratio (14 * timeinvest as u128 , 60*60*24*365 as  u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
        
            bonus = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
    
        if staker_info.tire == Uint128::new(4)
        {
           
            let total_profit_percentage= Decimal::from_ratio (18 * timeinvest as u128 , 60*60*24*365 as  u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            bonus = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

      }


     let decimal_amount=Uint128::new(1000000);
     
     let tire_0_amount = Uint128::new(20000);  
     let tire_1_amount = Uint128::new(60000); 
     let tire_2_amount = Uint128::new(99000) ;               
     let tire_3_amount_1 = Uint128::new(100000); 
     let tire_3_amount_2 =  Uint128::new(199999); 
     let tire_4_amount =  Uint128::new(200000);     
    
     let checked_amount = (amount + bonus)/decimal_amount;
     if checked_amount < tire_0_amount
  {
      staker_info.tire =Uint128::zero();
      let res = amount.multiply_ratio(7u128,10u128);
     let fee = Decimal::from_ratio(res , 100 as u128);
     let staking_fee = fee * Uint128::from(1 as u128);    
     staker_info.stake_amount += amount - staking_fee;
     staker_info.fee=staking_fee;

  }

  if  checked_amount >= tire_0_amount && checked_amount < tire_1_amount
  {
    staker_info.tire =Uint128::new(1);
    let res = amount.multiply_ratio(7u128,10u128);
   let fee = Decimal::from_ratio(res , 100 as u128);
   let staking_fee = fee * Uint128::from(1 as u128);    
   staker_info.stake_amount += amount - staking_fee;
   staker_info.fee=staking_fee;
  }

  if checked_amount >= tire_1_amount && checked_amount <= tire_2_amount
  {
    staker_info.tire =Uint128::new(2);
    let res = amount.multiply_ratio(5u128,10u128);
   let fee = Decimal::from_ratio(res , 100 as u128);
   let staking_fee = fee * Uint128::from(1 as u128);    
   staker_info.stake_amount += amount - staking_fee;
   staker_info.fee=staking_fee;
  }

 if checked_amount >= tire_3_amount_1 && checked_amount <= tire_3_amount_2
  {
    staker_info.tire =Uint128::new(3);  
    let res = amount.multiply_ratio(3u128,10u128);
    let fee = Decimal::from_ratio(res , 100 as u128);
    let staking_fee = fee * Uint128::from(1 as u128);    
    staker_info.stake_amount += amount - staking_fee;
    staker_info.fee=staking_fee;
  }

  if checked_amount >= tire_4_amount
  {
    staker_info.tire =Uint128::new(4);
    let res = amount.multiply_ratio(0u128,10u128);
   let fee = Decimal::from_ratio(res , 100 as u128);
   let staking_fee = fee * Uint128::from(1 as u128);    
   staker_info.stake_amount += amount - staking_fee;
   staker_info.fee=staking_fee;
  } 

  
  staker_info.start_time = current_time;
 
     store_staker_info(deps.storage, &sender_addr_raw, &staker_info)?;
    

    Ok(Response::new().add_attributes(vec![
        ("action", "bond"),
        ("staker_addr", &sender_addr.to_string()),
        ("amount", &amount.to_string()),
    ]))
}


/*
  locked funtion used for lock staking 
  this function will take three parameters 
  address, amount and month thesen months start 
  from 1 to 120 so thats time you set you cant withdraw 
  before this
            "locked json "


                {

              "send": {
               "msg": "eyJsb2NrZWQiOnsibW9udGgiOjF9fQ==",
               "amount": "1500000000",
               "contract": "terra13dycyqjf8kv0xqqlh2wm5lq98w3lzkptgrt9mj"
               }

              }

*/

pub fn locked(deps: DepsMut, env: Env, sender_addr: Addr, amount: Uint128,month:u64) -> StdResult<Response> {
    let current_time = get_time(&env.block);
    let sender_addr_raw: CanonicalAddr = deps.api.addr_canonicalize(sender_addr.as_str())?;

   
       if amount == Uint128::zero()
       {
           return Err(StdError::generic_err("Please enter correct amount"));
       }

   
    let mut staker_info: StakerLockedInfo = read_staker_locked_info(deps.storage, &sender_addr_raw)?;
      if staker_info.stake_amount > Uint128::zero()
      {
        return Err(StdError::generic_err("you already lock amount"));
      }

    let decimal_amount=Uint128::new(1000000);
    
    let tire_0_amount = Uint128::new(20000);  
    let tire_1_amount = Uint128::new(60000); 
    let tire_2_amount = Uint128::new(99000) ;          
    let tire_3_amount_1 = Uint128::new(100000);   
    let tire_3_amount_2 =  Uint128::new(199999);     
    let tire_4_amount =  Uint128::new(200000);               
   
    let checked_amount = (amount + staker_info.stake_amount)/decimal_amount;
    if checked_amount < tire_0_amount
    {
        staker_info.tire =Uint128::zero();
        let res = amount.multiply_ratio(7u128,10u128);
       let fee = Decimal::from_ratio(res , 100 as u128);
       let staking_fee = fee * Uint128::from(1 as u128);    
       staker_info.stake_amount += amount - staking_fee;
       staker_info.fee=staking_fee;
  
    }
  
    if  checked_amount >= tire_0_amount && checked_amount < tire_1_amount
    {
      staker_info.tire =Uint128::new(1);
      let res = amount.multiply_ratio(7u128,10u128);
     let fee = Decimal::from_ratio(res , 100 as u128);
     let staking_fee = fee * Uint128::from(1 as u128);    
     staker_info.stake_amount += amount - staking_fee;
     staker_info.fee=staking_fee;
    }
  
    if checked_amount >= tire_1_amount && checked_amount <= tire_2_amount
    {
      staker_info.tire =Uint128::new(2);
      let res = amount.multiply_ratio(5u128,10u128);
     let fee = Decimal::from_ratio(res , 100 as u128);
     let staking_fee = fee * Uint128::from(1 as u128);    
     staker_info.stake_amount += amount - staking_fee;
     staker_info.fee=staking_fee;
    }
  
   if checked_amount >= tire_3_amount_1 && checked_amount <= tire_3_amount_2
    {
      staker_info.tire =Uint128::new(3);  
      let res = amount.multiply_ratio(3u128,10u128);
      let fee = Decimal::from_ratio(res , 100 as u128);
      let staking_fee = fee * Uint128::from(1 as u128);    
      staker_info.stake_amount += amount - staking_fee;
      staker_info.fee=staking_fee;
    }
  
    if checked_amount >= tire_4_amount
    {
      staker_info.tire =Uint128::new(4);
      let res = amount.multiply_ratio(0u128,10u128);
     let fee = Decimal::from_ratio(res , 100 as u128);
     let staking_fee = fee * Uint128::from(1 as u128);    
     staker_info.stake_amount += amount - staking_fee;
     staker_info.fee=staking_fee;
    } 
  
    
    staker_info.start_time = current_time;
    staker_info.lock_end = current_time + month*(30*24*60*60);
    staker_info.month = month;



    store_staker_locked_info(deps.storage, &sender_addr_raw, &staker_info)?;

   Ok(Response::new().add_attributes(vec![
       ("action", "locked"),
       ("staker_addr", &sender_addr.to_string()),
       ("amount", &amount.to_string()),
   ]))
}



/*
this function is used to withdraw unlocked staking amount and get 
bonus according to the tire this withdraw function take 
one paramenter thats is amount_withdraw

            "withdraw unlocked json"

            {
                "withdraw":{
                    "amount_withdraw":"1000000"
                }
            }



*/

// withdraw rewards to executor
pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo,amount_withdraw: Uint128) -> StdResult<Response> {
    let current_time = get_time(&env.block);
    let sender_addr_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
     let mut  amount=Uint128::zero();
    let config: Config = read_config(deps.storage)?;
    let mut staker_info = read_staker_info(deps.storage, &sender_addr_raw)?;
    let timeinvest = current_time - staker_info.start_time;
   let decimal_value=Uint128::new (1000000);

    if amount_withdraw == Uint128::zero()
    {
        return Err(StdError::generic_err("Please enter correct amount"));
    }
   
    if staker_info.tire == Uint128::zero()
    {  
      amount = staker_info.stake_amount - amount_withdraw;
    }

    if staker_info.tire == Uint128::new(1)
    {
     
    
     let total_profit_percentage = Decimal::from_ratio (10 * timeinvest as u128 ,60*60*24*365 as u128);
     let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
     amount = (staker_info.stake_amount + (total_value  * Uint128::from(1 as u128)))-amount_withdraw;
    }

    if staker_info.tire == Uint128::new(2)
    {
        let total_profit_percentage= Decimal::from_ratio (12 * timeinvest as u128, 60*60*24*365 as  u128);
        let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
        amount = (staker_info.stake_amount + (total_value  * Uint128::from(1 as u128)))-amount_withdraw;    }

    if staker_info.tire == Uint128::new(3)
    {
        
        let total_profit_percentage= Decimal::from_ratio (14 * timeinvest as u128 , 60*60*24*365 as  u128);
        let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
    
        amount = (staker_info.stake_amount + (total_value  * Uint128::from(1 as u128)))-amount_withdraw;
    }

    if staker_info.tire == Uint128::new(4)
    {
       
        let total_profit_percentage= Decimal::from_ratio (18 * timeinvest as u128 , 60*60*24*365 as  u128);
        let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
        amount = (staker_info.stake_amount + (total_value  * Uint128::from(1 as u128)))-amount_withdraw;
    }


    if amount > Uint128::zero()
    {

        staker_info.stake_amount=amount;
        staker_info.start_time=current_time;
    }
    else{

    
    staker_info.stake_amount=Uint128::zero();
    staker_info.start_time=0;
    staker_info.tire=Uint128::zero();
    staker_info.fee=Uint128::zero();

    }
    
   
  store_staker_info(deps.storage, &sender_addr_raw, &staker_info)?;

    Ok(Response::new()
        .add_message(WasmMsg::Execute {
            contract_addr: deps.api.addr_humanize(&config.staking_token)?.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount:amount_withdraw,
            })?,
            funds: vec![],
        })
        .add_attributes(vec![
            ("action", "withdraw"),
            ("owner", &info.sender.to_string()),
            ("amount", &amount_withdraw.to_string()),
        ]))
}


/*
this withdraw locked function used to withdraw 
locked amount if duration paeriod complete and amount is unlocke 
this function not take any parameter just withdraw your amount
 
                     "withdraw_locked json"

                     {
                         "withdraw_locked":{}
                     }
                
                     

*/



pub fn withdraw_locked (deps: DepsMut, env: Env, info: MessageInfo,) -> StdResult<Response> {
    let current_time = get_time(&env.block);
    let sender_addr_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
     let mut  amount=Uint128::zero();
    let config: Config = read_config(deps.storage)?;
    
    let mut staker_info = read_staker_locked_info(deps.storage, &sender_addr_raw)?;
   let timeinvest =  staker_info.start_time - current_time;

   let decimal_value=Uint128::new (1000000);

    
     
    if current_time > staker_info.lock_end
    {
        return Err(StdError::generic_err("your locked time not end yet"));
    }
   
    if staker_info.tire == Uint128::zero()
    {  
      amount = staker_info.stake_amount;
    //  amount =Decimal:: multiply_ratio(staker_info.stake_amount,1000);
    }
   
    if staker_info.tire == Uint128::new(1)
    {
        if staker_info.month == 1
        {
            let total_profit_percentage= Decimal::from_ratio (10 * timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
        if staker_info.month == 3
        {
            let total_profit_percentage= Decimal::from_ratio (11* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
        if staker_info.month == 6
        {
            let total_profit_percentage= Decimal::from_ratio (12* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 12
        {
            let total_profit_percentage= Decimal::from_ratio (13* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 24
        {
            let total_profit_percentage= Decimal::from_ratio (14* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 36
        {
            let total_profit_percentage= Decimal::from_ratio (15* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 60
        {
            let total_profit_percentage= Decimal::from_ratio (16* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 120
        {
            let total_profit_percentage= Decimal::from_ratio (17* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
     
    // let percentage_per_sec= 10/(60*60*24*365);
    
    }

    if staker_info.tire == Uint128::new(2)
    {
        if staker_info.month == 1
        {
            let total_profit_percentage= Decimal::from_ratio (12 * timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
        if staker_info.month == 3
        {
            let total_profit_percentage= Decimal::from_ratio (13* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
        if staker_info.month == 6
        {
            let total_profit_percentage= Decimal::from_ratio (14* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 12
        {
            let total_profit_percentage= Decimal::from_ratio (15* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 24
        {
            let total_profit_percentage= Decimal::from_ratio (16* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 36
        {
            let total_profit_percentage= Decimal::from_ratio (17* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 60
        {
            let total_profit_percentage= Decimal::from_ratio (18* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 120
        {
            let total_profit_percentage= Decimal::from_ratio (19* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
     
    }

    if staker_info.tire == Uint128::new(3)
    {
        if staker_info.month == 1
        {
            let total_profit_percentage= Decimal::from_ratio (14 * timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
        if staker_info.month == 3
        {
            let total_profit_percentage= Decimal::from_ratio (15* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
        if staker_info.month == 6
        {
            let total_profit_percentage= Decimal::from_ratio (16* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 12
        {
            let total_profit_percentage= Decimal::from_ratio (18* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 24
        {
            let total_profit_percentage= Decimal::from_ratio (20* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 36
        {
            let total_profit_percentage= Decimal::from_ratio (21* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 60
        {
            let total_profit_percentage= Decimal::from_ratio (22* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 120
        {
            let total_profit_percentage= Decimal::from_ratio (25* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
    }

    if staker_info.tire == Uint128::new(4)
    {
        if staker_info.month == 1
        {
            let total_profit_percentage= Decimal::from_ratio (18 * timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
        if staker_info.month == 3
        {
            let total_profit_percentage= Decimal::from_ratio (20* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
        if staker_info.month == 6
        {
            let total_profit_percentage= Decimal::from_ratio (22* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 12
        {
            let total_profit_percentage= Decimal::from_ratio (24* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 24
        {
            let total_profit_percentage= Decimal::from_ratio (26* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 36
        {
            let total_profit_percentage= Decimal::from_ratio (28* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 60
        {
            let total_profit_percentage= Decimal::from_ratio (30* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }

        if staker_info.month == 120
        {
            let total_profit_percentage= Decimal::from_ratio (50* timeinvest as u128 ,60*60*24*365 as u128);
            let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
            amount = staker_info.stake_amount + (total_value  * Uint128::from(1 as u128));
        }
    }

    staker_info.stake_amount=Uint128::zero();
    staker_info.start_time=0;
    staker_info.tire=Uint128::zero();
    staker_info.fee=Uint128::zero();
    staker_info.month=0;
    staker_info.lock_end=0;

  store_staker_locked_info(deps.storage, &sender_addr_raw, &staker_info)?;

    Ok(Response::new()
        .add_message(WasmMsg::Execute {
            contract_addr: deps.api.addr_humanize(&config.staking_token)?.to_string(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: info.sender.to_string(),
                amount,
            })?,
            funds: vec![],
        })
        .add_attributes(vec![
            ("action", "withdraw_locked"),
            ("owner", &info.sender.to_string()),
            ("amount", &amount.to_string()),
        ]))
}

/*  
     withdraw_owner this function used  to withdraw token in staking sunction
     only owner can execute this function this function only take one parameter
     that is amount.

          "withdraw_owner json"

          {
              "withdraw_owner":{
                  "amount":1000000
              }
          }

     */


pub fn withdraw_owner(deps: DepsMut, env: Env, info: MessageInfo, amount: Uint128) -> StdResult<Response>
{
    let config: Config = read_config(deps.storage)?;
    if config.ownership != info.sender
    {
        return Err(StdError::generic_err("owner can execute this function"));
    }

    Ok(Response::new()
    .add_message(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(&config.staking_token)?.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    })
    .add_attributes(vec![
        ("action", "ownerwithdraw"),
        ("owner", &info.sender.to_string()),
        ("amount", &amount.to_string()),
    ]))

}


fn get_time(block: &BlockInfo) -> u64 {
    block.time.seconds()
}


/* execute_transfer_usd is custom function of token 
   in which only owner can witdraw usd from smart contract. owner
   need to pass amount in six decimal how much want to withdraw

                   "execute transfer usd json"


                    {
                     "transfer_usd":{
                         "amount":"1000000"
                     }
                        
                    }
   
   */



pub fn execute_transfer_usd(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount:Uint128,

) -> StdResult<Response>
{
   
    let config: Config = read_config(deps.storage)?;
    if config.ownership != info.sender 
    {
        return Err(StdError::generic_err("owner can execute this function"));
    } 
  
  let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom:"uusd".to_string(),
            amount: amount,
        }],
    }) as CosmosMsg ;

    let res = Response::new();

 Ok(res.add_message(msg))
    

}


/* execute_transfer_luna is custom function of token 
   in which only owner can witdraw  luna  from smart contract. owner
   need to pass amount in six decimal how much want to withdraw

                    "execute transfer luna json"


                     {
                     "transfer_luna":{
                         "amount":"1000000"
                     }
                        
                    }
   
   
   */




pub fn execute_transfer_luna(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount:Uint128,

) -> StdResult<Response>
{
   
    let config: Config = read_config(deps.storage)?;
    if config.ownership != info.sender 
    {
        return Err(StdError::generic_err("owner can execute this function"));
    } 
  
  let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom:"uluna".to_string(),
            amount: amount,
        }],
    }) as CosmosMsg ;

    let res = Response::new();

 Ok(res.add_message(msg))
    
}

 /*
   "config query info json"
   {
       "staking_info":{}
   }

   */


   /*
         "staker info json "

         {
             "staker_info":{
                 "staker_address":"0x17"
             }
         }

   */



      /*
         "staker locked_info json query "

         {
             "staker_locked_info":{
                 "staker_address":"0x17"
             }
         }

   */

   


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        
        QueryMsg::StakerInfo {
            staker_address,

        } => to_binary(&query_staker_info(deps,_env, staker_address)?),

        QueryMsg::StakerLockedInfo {
            staker_address,

        } => to_binary(&query_staker_locked_info(deps, staker_address)?),
    }
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = read_config(deps.storage)?;
    let resp = ConfigResponse {
       
        staking_token: deps.api.addr_humanize(&state.staking_token)?.to_string(),
        ownership:state.ownership,
       
    };

    Ok(resp)
}


pub fn query_staker_info(
    deps: Deps,
    env: Env,
    staker: String,
) -> StdResult<StakerInfoResponse> {
    let current_time = get_time(&env.block);
    let staker_raw = deps.api.addr_canonicalize(&staker)?;

    let mut staker_info: StakerInfo = read_staker_info(deps.storage, &staker_raw)?;
    let timeinvest = current_time - staker_info.start_time;
    let mut  bonus = Uint128::zero();

    if staker_info.tire == Uint128::zero()
    {  
        bonus = Uint128::zero();
    
    }

    if staker_info.tire == Uint128::new(1)
    {
     
     let total_profit_percentage = Decimal::from_ratio (10 * timeinvest as u128 ,60*60*24*365 as u128);
     let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
     bonus = total_value  * Uint128::from(1 as u128);
    }

    if staker_info.tire == Uint128::new(2)
    {
        
        let total_profit_percentage= Decimal::from_ratio (12 * timeinvest as u128, 60*60*24*365 as  u128);
        let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
        bonus = total_value  * Uint128::from(1 as u128);
    }

    if staker_info.tire == Uint128::new(3)
    {
        
        let total_profit_percentage= Decimal::from_ratio (14 * timeinvest as u128 , 60*60*24*365 as  u128);
        let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
    
        bonus = total_value  * Uint128::from(1 as u128);
    }

    if staker_info.tire == Uint128::new(4)
    {
       
        let total_profit_percentage= Decimal::from_ratio (18 * timeinvest as u128 , 60*60*24*365 as  u128);
        let total_value=Decimal::from_ratio (total_profit_percentage  *staker_info.stake_amount,Uint128::new(100));
        bonus = total_value  * Uint128::from(1 as u128);
    }
  
    Ok(StakerInfoResponse {
        staker_address:staker,
        stake_amount: staker_info.stake_amount,
        start_time: staker_info.start_time,
        tire:staker_info.tire,
        fee:staker_info.fee,
        bonus:bonus,
       
    })
}
pub fn query_staker_locked_info(
    deps: Deps,
    staker: String,
) -> StdResult<StakerLockedInfoResponse> {
    let staker_raw = deps.api.addr_canonicalize(&staker)?;

    let mut staker_info: StakerLockedInfo = read_staker_locked_info(deps.storage, &staker_raw)?;
   

    Ok(StakerLockedInfoResponse {
        staker_address:staker,
        stake_amount: staker_info.stake_amount,
        start_time: staker_info.start_time,
        tire:staker_info.tire,
        month:staker_info.month,
        fee:staker_info.fee,
        lock_end:staker_info.lock_end,
       
    })
}


