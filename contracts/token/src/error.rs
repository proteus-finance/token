use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Allowance is expired")]
    Expired {},

    #[error("Invalid Amount Entered")]
    Invalid {},

    #[error("first three Months not completed")]
    InvalidTime {},

    #[error("seed amount exceed")]
    InvalidAmountSeed {},

    #[error("amount finish for 1 month")]
    InvalidAmountFinish {},
    
    #[error("No allowance for this account")]
    NoAllowance {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Logo binary data exceeds 5KB limit")]
    LogoTooBig {},

    #[error("Invalid xml preamble for SVG")]
    InvalidXmlPreamble {},

    #[error("Invalid png header")]
    InvalidPngHeader {},

    #[error("Invalid Liquidity Amount Entered")]
    InvalidLiquidity {},

    #[error("Time Ended")]
    TimeEnd {},

    #[error("6 Month not Completed")]
    TimeNotComplete {},

    #[error("Wrong IDO Duration")]
    Idoduration{},

    #[error("the price of token is higher")]
    PriceToken {},

    #[error("Invalid Amount")]
    InvalidAmount {},

}
