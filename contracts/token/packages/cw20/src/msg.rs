use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::logo::Logo;
use cosmwasm_std::{Binary, Uint128,Addr};
use cw0::Expiration;
use cosmwasm_std::CanonicalAddr;
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Cw20ExecuteMsg {
    /// Seed is a base message to move tokens to another account without triggering actions
    Seed { investors:Vec<InvestorInfo>},

    Liquidity { recipient: String, amount: Uint128 },

    Advisor { recipient: String, amount: Uint128 },

    Community { recipient: String, amount: Uint128 },

    Team { recipient: String, amount: Uint128 },

    Transfer { recipient: String, amount: Uint128 },

    Insurance { recipient: String, amount: Uint128 },

    Staking { recipient: String, amount: Uint128 },

    Request { recipient: String, amount: Uint128 },

    Public { recipient: String,amount:Uint128},


    ChangeOwner{owner_address:Addr},

    /// Burn is a base message to destroy tokens forever
    Burn { amount: Uint128 },
    /// Send is a base message to transfer tokens to a contract and trigger an action
    /// on the receiving contract.
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with "approval" extension. Allows spender to access an additional amount tokens
    /// from the owner's (env.sender) account. If expires is Some(), overwrites current allowance
    /// expiration with this one.
    IncreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },

    TransferUsd{
     amount:Uint128,
    },

    Claim{},


    TransferLuna{
        amount:Uint128,
       },
    /// Only with "approval" extension. Lowers the spender's access of tokens
    /// from the owner's (env.sender) account by amount. If expires is Some(), overwrites current
    /// allowance expiration with this one.
    DecreaseAllowance {
        spender: String,
        amount: Uint128,
        expires: Option<Expiration>,
    },
    /// Only with "approval" extension. Transfers amount tokens from owner -> recipient
    /// if `env.sender` has sufficient pre-approval.
    TransferFrom {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    /// Only with "approval" extension. Sends amount tokens from owner -> contract
    /// if `env.sender` has sufficient pre-approval.
    SendFrom {
        owner: String,
        contract: String,
        amount: Uint128,
        msg: Binary,
    },
    /// Only with "approval" extension. Destroys tokens forever
    BurnFrom { owner: String, amount: Uint128 },
    /// Only with the "mintable" extension. If authorized, creates amount new tokens
    /// and adds to the recipient balance.
    Mint { recipient: String, amount: Uint128 },
    /// Only with the "marketing" extension. If authorized, updates marketing metadata.
    /// Setting None/null for any of these will leave it unchanged.
    /// Setting Some("") will clear this field on the contract storage
    UpdateMarketing {
        /// A URL pointing to the project behind this token.
        project: Option<String>,
        /// A longer description of the token and it's utility. Designed for tooltips or such
        description: Option<String>,
        /// The address (if any) who can update this data structure
        marketing: Option<String>,
    },
    /// If set as the "marketing" role on the contract, upload a new URL, SVG, or PNG for the token
    UploadLogo(Logo),
}

 #[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InvestorInfo {
    pub investor:String,
    pub amount_given: Uint128,
    pub witdraw:Uint128,
    pub last_time_withdraw:u64,
    pub amount_remain:Uint128,
    pub user_invest_time:u64,
    pub first_claim:Uint128,
    pub perday_amount:Uint128,

}