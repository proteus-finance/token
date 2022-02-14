use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use cw20::{AllowanceResponse, Logo, MarketingInfoResponse};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub mint: Option<MinterData>,
    pub seed_token_sale: Uint128,
    pub ido: Uint128,
    pub staking_funds: Uint128,
    pub insurance_funds: Uint128,
    pub team: Uint128,
    pub advisors: Uint128,
    pub launch_pad: Uint128,
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
    pub launch_pad_start_month:u64,
    pub launch_pad_end_month:u64,
    pub launch_pad_next_month:u64,
    pub launch_pad_amount_monthly:Uint128,
    pub launch_pad_amount_remain:Uint128,
    pub team_start_month:u64,
    pub team_end_month:u64,
    pub team_next_month:u64,
    pub team_amount_monthly:Uint128,
    pub team_amount_monthly_remain:Uint128,
    pub ido_start_month:u64,
    pub ido_end_month:u64,

}

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

pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const MARKETING_INFO: Item<MarketingInfoResponse> = Item::new("marketing_info");
pub const LOGO: Item<Logo> = Item::new("logo");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub const ALLOWANCES: Map<(&Addr, &Addr), AllowanceResponse> = Map::new("allowance");