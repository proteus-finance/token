use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Binary, Uint128};

use crate::logo::LogoInfo;
use cw0::Expiration;
use cosmwasm_std::CanonicalAddr;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Cw20QueryMsg {
    /// Returns the current balance of the given address, 0 if unset.
    /// Return type: BalanceResponse.
    Balance { address: String },



    InvestorInfo{address:String} ,   
    /// Returns metadata on the contract - name, decimals, supply, etc.
    /// Return type: TokenInfoResponse.
    TokenInfo {},
    /// Only with "allowance" extension.
    /// Returns how much spender can use from owner account, 0 if unset.
    /// Return type: AllowanceResponse.
    Allowance { owner: String, spender: String },
    /// Only with "mintable" extension.
    /// Returns who can mint and the hard cap on maximum tokens after minting.
    /// Return type: MinterResponse.
    Minter {},
    /// Only with "marketing" extension
    /// Returns more metadata on the contract to display in the client:
    /// - description, logo, project url, etc.
    /// Return type: MarketingInfoResponse.
    MarketingInfo {},
    /// Only with "marketing" extension
    /// Downloads the embedded logo data (if stored on chain). Errors if no logo data stored for
    /// this contract.
    /// Return type: DownloadLogoResponse.
    DownloadLogo {},
    /// Only with "enumerable" extension (and "allowances")
    /// Returns all allowances this owner has approved. Supports pagination.
    /// Return type: AllAllowancesResponse.
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Only with "enumerable" extension
    /// Returns all accounts that have balances. Supports pagination.
    /// Return type: AllAccountsResponse.
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub supply_limit:Uint128,
    pub seed_token_sale: Uint128,
    pub public: Uint128,
    pub staking_funds: Uint128,
    pub insurance_funds: Uint128,
    pub team: Uint128,
    pub advisors: Uint128,
    pub community: Uint128,
    pub liquidity: Uint128,
    pub owner: Addr,
    pub next_month:u64,
    pub end_time:u64,
    pub start_month:u64,
    pub three_month_period:u64,
    pub start_month_advisor:u64,
    pub next_month_advisor:u64,
    pub end_month_advisor:u64,
    pub monthly_advisor_amount_remain:Uint128,
    pub monthly_advisor_amount:Uint128,
    pub team_start_month:u64,
    pub team_end_month:u64,
    pub team_next_month:u64,
    pub team_amount_monthly:Uint128,
    pub team_amount_monthly_remain:Uint128,
    pub total_supply: Uint128,
    pub team_last_month_amount: Uint128,
    pub team_last_month: u64,

}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct AllowanceResponse {
    pub allowance: Uint128,
    pub expires: Expiration,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MinterResponse {
    pub minter: String,
    /// cap is a hard cap on total supply that can be achieved by minting.
    /// Note that this refers to total_supply.
    /// If None, there is unlimited cap.
    pub cap: Option<Uint128>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct MarketingInfoResponse {
    /// A URL pointing to the project behind this token.
    pub project: Option<String>,
    /// A longer description of the token and it's utility. Designed for tooltips or such
    pub description: Option<String>,
    /// A link to the logo, or a comment there is an on-chain logo stored
    pub logo: Option<LogoInfo>,
    /// The address (if any) who can update this data structure
    pub marketing: Option<Addr>,
}

/// When we download an embedded logo, we get this response type.
/// We expect a SPA to be able to accept this info and display it.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct DownloadLogoResponse {
    pub mime_type: String,
    pub data: Binary,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AllowanceInfo {
    pub spender: String,
    pub allowance: Uint128,
    pub expires: Expiration,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct AllAllowancesResponse {
    pub allowances: Vec<AllowanceInfo>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct AllAccountsResponse {
    pub accounts: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InvestorInfoResponse {
    pub investor:String,
    pub amount_given: Uint128,
    // pub last_time_withdraw:u64,
    // pub amount_remain:Uint128,
    // pub perday_amount:Uint128,
    // pub user_invest_time:u64,
    // pub first_claim:Uint128,
}
