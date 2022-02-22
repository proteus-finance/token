use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Decimal, Uint128,CosmosMsg,Coin,BankMsg};
use cw20::Cw20ReceiveMsg;
use terraswap::asset::Asset;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub staking_token: String, // lp token of PSI-UST or nAsset-PSI pair contract
    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    
    // Unbond {
    //     amount: Uint128,
    // },
    // Withdraw pending rewards
     Withdraw {amount_withdraw:Uint128},

     WithdrawLocked {},

     WithdrawOwner {amount:Uint128},

     TransferUsd {amount:Uint128},

     TransferLuna {amount:Uint128},
   
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    Bond {},

   Locked{month:u64},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    // State {
    //     time_seconds: Option<u64>,
    // },
    StakerInfo {
        staker_address: String,
    },
    StakerLockedInfo {
        staker_address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub staking_token: String,
    pub ownership:String,
    
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct StateResponse {
//     pub last_distributed: u64,
//     pub total_bond_amount: Uint128,
//     pub global_reward_index: Decimal,
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfoResponse {
    pub staker_address:String,
    pub stake_amount:Uint128,
    pub start_time:u64,
    pub tire:Uint128,
    pub fee:Uint128,
    pub bonus:Uint128,
    
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerLockedInfoResponse {
    pub staker_address:String,
    pub stake_amount:Uint128,
    pub start_time:u64,
    pub tire:Uint128,
    pub month:u64,
    pub fee:Uint128,
    pub lock_end:u64,
    
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct MigrateMsg {}
