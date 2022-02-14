use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Decimal, StdResult, Storage, Uint128};
use cosmwasm_storage::{singleton, singleton_read, Bucket, ReadonlyBucket};
//use services::staking::StakingSchedule;

static KEY_CONFIG: &[u8] = b"config";
static KEY_STATE: &[u8] = b"state";

static PREFIX_REWARD: &[u8] = b"reward";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
   // pub owner: CanonicalAddr,
   // pub psi_token: CanonicalAddr,
    //pub terraswap_factory: CanonicalAddr,
    pub staking_token: CanonicalAddr,
    pub ownership:String,
   // pub distribution_schedule: Vec<StakingSchedule>,
}

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    singleton(storage, KEY_CONFIG).save(config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    singleton_read(storage, KEY_CONFIG).load()
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerInfo {
   pub staker_address: String,
   pub stake_amount: Uint128,
   pub start_time:u64,
   pub tire:Uint128,
   pub fee: Uint128,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StakerLockedInfo {
    pub staker_address: String,
    pub stake_amount: Uint128,
    pub start_time:u64,
    pub tire:Uint128,
    pub month:u64,
    pub fee: Uint128,
 }

//returns return staker_info of the given owner
pub fn store_staker_info(
    storage: &mut dyn Storage,
    owner: &CanonicalAddr,
    staker_info: &StakerInfo,
) -> StdResult<()> {
    Bucket::new(storage, PREFIX_REWARD).save(owner.as_slice(), staker_info)
}


pub fn store_staker_locked_info(
    storage: &mut dyn Storage,
    owner: &CanonicalAddr,
    staker_locked_info: &StakerLockedInfo,
) -> StdResult<()> {
    Bucket::new(storage, PREFIX_REWARD).save(owner.as_slice(), staker_locked_info)
}



// remove staker_info of the given owner
// pub fn remove_staker_info(storage: &mut dyn Storage, owner: &CanonicalAddr) {
//     Bucket::<StakerInfo>::new(storage, PREFIX_REWARD).remove(owner.as_slice())
// }

//returns rewards owned by this owner
//(read-only version for queries)
pub fn read_staker_info(storage: &dyn Storage, owner: &CanonicalAddr) -> StdResult<StakerInfo> {
    match ReadonlyBucket::new(storage, PREFIX_REWARD).may_load(owner.as_slice())? {
        Some(staker_info) => Ok(staker_info),
        None => Ok(StakerInfo {
            staker_address:owner.to_string(),
            stake_amount:Uint128::zero(),
            start_time:0,
            tire:Uint128::zero(),
            fee:Uint128::zero(),
        }),
    }
}

pub fn read_staker_locked_info(storage: &dyn Storage, owner: &CanonicalAddr) -> StdResult<StakerLockedInfo> {
    match ReadonlyBucket::new(storage, PREFIX_REWARD).may_load(owner.as_slice())? {
        Some(staker_locked_info) => Ok(staker_locked_info),
        None => Ok(StakerLockedInfo {
            staker_address:owner.to_string(),
            stake_amount:Uint128::zero(),
            start_time:0,
            tire:Uint128::zero(),
            month:0,
            fee:Uint128::zero(),
        }),
    }
}
