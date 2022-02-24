use cosmwasm_std::StdError;
use thiserror::Error;
// there we define the errors there
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    //if ownership not matched then unathorized 
    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},
    
    #[error("Allowance is expired")]
    Expired {},
     // if you entered zero amount then invalid amount entered will show
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
   // if condition is not completed then this eoor will show
    #[error("6 Month not Completed")]
    TimeNotComplete {},

    #[error("Wrong IDO Duration")]
    Idoduration{},

    #[error("the price of token is higher")]
    PriceToken {},

}
