use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use cw20::{AllowanceResponse, Logo, MarketingInfoResponse};
use cw20::msg::{InvestorInfo};
use cosmwasm_storage::{singleton, singleton_read, Bucket, ReadonlyBucket};
use cosmwasm_std::StdResult;
use cosmwasm_std::CanonicalAddr;
use cosmwasm_std::Storage;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub mint: Option<MinterData>,
    pub seed_token_sale: Uint128,
    pub public: Uint128,
    pub staking_funds: Uint128,
    pub insurance_funds: Uint128,
    pub team: Uint128,
    pub advisors: Uint128,
    pub community: Uint128,
    pub liquidity: Uint128,
    pub owner: Addr,
    pub start_month:u64,
    pub next_month:u64,
    pub end_time:u64,
    pub monthly_seed:Uint128,
    pub three_month_period:u64,
    pub monthly_seed_remain:Uint128,
    pub start_month_advisor:u64,
    pub next_month_advisor:u64,
    pub end_month_advisor:u64,
    pub monthly_advisor_amount:Uint128,
    pub monthly_advisor_amount_remain:Uint128,
    pub community_start_month:u64,
    pub community_end_month:u64,
    pub community_next_month:u64,
    pub community_amount_monthly:Uint128,
    pub community_amount_remain:Uint128,
    pub team_start_month:u64,
    pub team_end_month:u64,
    pub team_next_month:u64,
    pub team_amount_monthly:Uint128,
    pub team_amount_monthly_remain:Uint128,
    pub public_start_month:u64,
    pub public_end_month:u64,
    pub supply_limit:Uint128,
}

static PREFIX_REWARD: &[u8] = b"reward";


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MinterData {
    pub minter: Addr,
    /// cap is how many more tokens can be issued by the minter
    pub cap: Option<Uint128>,
}

impl TokenInfo {
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|v| v.cap)
    }
}

pub fn store_investor_info(
    storage: &mut dyn Storage,
    owner: &CanonicalAddr,
    investor_info: &InvestorInfo,
) -> StdResult<()> {
    Bucket::new(storage, PREFIX_REWARD).save(owner, investor_info)
}

pub fn read_investor_info(storage: &dyn Storage, owner:&CanonicalAddr) -> StdResult<InvestorInfo> {
    match ReadonlyBucket::new(storage, PREFIX_REWARD).may_load(owner.as_slice())? {
        Some(investor_info) => Ok(investor_info),
        None => Ok(InvestorInfo {
            investor:owner.to_string(),
            amount:Uint128::zero(),
            witdraw:Uint128::zero(),
            perday_amount:Uint128::zero(),
            last_time_withdraw:0,
            amount_remain:Uint128::zero(),
            user_invest_time:0,

        }),
    }
}


pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const MARKETING_INFO: Item<MarketingInfoResponse> = Item::new("marketing_info");
pub const LOGO: Item<Logo> = Item::new("logo");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub const ALLOWANCES: Map<(&Addr, &Addr), AllowanceResponse> = Map::new("allowance");
