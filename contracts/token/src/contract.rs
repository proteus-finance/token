#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,Addr,BankMsg,Coin,CosmosMsg
};
use cw2::set_contract_version;
use cw20::{
    BalanceResponse, Cw20Coin, Cw20ReceiveMsg, DownloadLogoResponse, EmbeddedLogo, Logo, LogoInfo,
    MarketingInfoResponse, MinterResponse, TokenInfoResponse,
};
use cw20::msg::{InvestorInfo};
use cw20::query::{InvestorInfoResponse};
use crate::allowances::{
    execute_burn_from, execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from, query_allowance,
};
use crate::enumerable::{query_all_accounts, query_all_allowances};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{MinterData, TokenInfo, BALANCES, LOGO, MARKETING_INFO, TOKEN_INFO,read_investor_info};
use crate::state::store_investor_info;
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const LOGO_SIZE_CAP: usize = 5 * 1024;


/// Checks if data starts with XML preamble
fn verify_xml_preamble(data: &[u8]) -> Result<(), ContractError> {
    // The easiest way to perform this check would be just match on regex, however regex
    // compilation is heavy and probably not worth it.

    let preamble = data
        .split_inclusive(|c| *c == b'>')
        .next()
        .ok_or(ContractError::InvalidXmlPreamble {})?;

    const PREFIX: &[u8] = b"<?xml ";
    const POSTFIX: &[u8] = b"?>";

    if !(preamble.starts_with(PREFIX) && preamble.ends_with(POSTFIX)) {
        Err(ContractError::InvalidXmlPreamble {})
    } else {
        Ok(())
    }

    // Additionally attributes format could be validated as they are well defined, as well as
    // comments presence inside of preable, but it is probably not worth it.
}

/// Validates XML logo
fn verify_xml_logo(logo: &[u8]) -> Result<(), ContractError> {
    verify_xml_preamble(logo)?;

    if logo.len() > LOGO_SIZE_CAP {
        Err(ContractError::LogoTooBig {})
    } else {
        Ok(())
    }
}

/// Validates png logo
fn verify_png_logo(logo: &[u8]) -> Result<(), ContractError> {
    // PNG header format:
    // 0x89 - magic byte, out of ASCII table to fail on 7-bit systems
    // "PNG" ascii representation
    // [0x0d, 0x0a] - dos style line ending
    // 0x1a - dos control character, stop displaying rest of the file
    // 0x0a - unix style line ending
    const HEADER: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
    if logo.len() > LOGO_SIZE_CAP {
        Err(ContractError::LogoTooBig {})
    } else if !logo.starts_with(&HEADER) {
        Err(ContractError::InvalidPngHeader {})
    } else {
        Ok(())
    }
}

/// Checks if passed logo is correct, and if not, returns an error
fn verify_logo(logo: &Logo) -> Result<(), ContractError> {
    match logo {
        Logo::Embedded(EmbeddedLogo::Svg(logo)) => verify_xml_logo(&logo),
        Logo::Embedded(EmbeddedLogo::Png(logo)) => verify_png_logo(&logo),
        Logo::Url(_) => Ok(()), // Any reasonable url validation would be regex based, probably not worth it
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;
    // create initial accounts
    let total_supply = create_accounts(&mut deps, &msg.initial_balances)?;
    if let Some(limit) = msg.get_cap() {
        if total_supply > limit {
            return Err(StdError::generic_err("Initial supply greater than cap").into());
        }
        
    }
    
   
     

    let _cap = msg.supply_limit;
    let _seed_token_sale = (_cap * Uint128::new(9))/Uint128::new(100);
    let _ido = (_cap * Uint128::new(4))/Uint128::new(100);
    let _staking_funds = (_cap * Uint128::new(25))/Uint128::new(100);
    let _insurance_funds = (_cap * Uint128::new(20))/Uint128::new(100);
    let _team = (_cap * Uint128::new(17))/Uint128::new(100);
    let _advisors = (_cap * Uint128::new(2))/Uint128::new(100);
    let _launch_pad = (_cap * Uint128::new(17))/Uint128::new(100);
    let _liquidity = (_cap * Uint128::new(6))/Uint128::new(100);
    let _start_time=_env.block.time.seconds() + 3*30*24*60*60;
    let _end_time=_env.block.time.seconds() + 18*30*24*60*60;
    let _mothly_seed=(_seed_token_sale * Uint128::new(5))/Uint128::new(100);
    let _three_month_period=_env.block.time.seconds() + 3*30*24*60*60;
    let _next_month=_env.block.time.seconds() + 4*30*24*60*60;
    let  _monthly_advisor_amount=(_advisors * Uint128::new(10))/Uint128::new(100);
    let  _end_month_advisor = _env.block.time.seconds() + 13*30*24*60*60;
    let _launch_pad_amount_monthly =  (_launch_pad *Uint128::new(10))/Uint128::new(100);
    let _launch_pad_end_month=_env.block.time.seconds() + 16*30*24*60*60;
    let _launch_pad_next_month=_env.block.time.seconds() + 7*30*24*60*60;
    let _launch_pad_start_month=_env.block.time.seconds() + 6*30*24*60*60;
    let _team_end_month = _env.block.time.seconds() + 16*30*24*60*60;
    let _team_amount_monthly = (_team *Uint128::new(10))/Uint128::new(100);
    let _team_next_month = _env.block.time.seconds() + 7*30*24*60*60;
    let _ido_start_month = _env.block.time.seconds() + 6*30*24*60*60 ;
    let _ido_end_month   =   _env.block.time.seconds() + 8*30*24*60*60;


    let mint = match msg.mint {
        Some( m) => Some(MinterData {
            minter: deps.api.addr_validate(&m.minter)?,
            cap: m.cap,
        }),
        None => None,

    };


    // store token info
    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply,
        mint,
        seed_token_sale: _seed_token_sale,
        ido: _ido,
        staking_funds: _staking_funds,
        insurance_funds: _insurance_funds,
        team: _team,
        advisors: _advisors,
        launch_pad: _launch_pad,
        liquidity: _liquidity,
        owner:_info.sender,
        end_time:_end_time,
        monthly_seed:_mothly_seed,
        monthly_seed_remain:_mothly_seed,
        three_month_period:_three_month_period,
        start_month:_start_time,
        next_month:_next_month,
        start_month_advisor:_start_time,
        next_month_advisor:_next_month,
        monthly_advisor_amount:_monthly_advisor_amount,
        monthly_advisor_amount_remain:_monthly_advisor_amount,
        end_month_advisor:_end_month_advisor,
        launch_pad_amount_monthly:_launch_pad_amount_monthly,
        launch_pad_amount_remain:_launch_pad_amount_monthly,
        launch_pad_start_month:_launch_pad_start_month,
        launch_pad_next_month:_launch_pad_next_month,
        launch_pad_end_month:_launch_pad_end_month,
        team_start_month:_launch_pad_start_month,
        team_end_month:_team_end_month,
        team_next_month:_team_next_month,
        team_amount_monthly:_team_amount_monthly,
        team_amount_monthly_remain:_team_amount_monthly,
        ido_start_month:_ido_start_month,
        ido_end_month:_ido_end_month,
        supply_limit:msg.supply_limit,
        
    };
    TOKEN_INFO.save(deps.storage, &data)?;

   

    if let Some(marketing) = msg.marketing {
        let logo = if let Some(logo) = marketing.logo {
            verify_logo(&logo)?;
            LOGO.save(deps.storage, &logo)?;

            match logo {
                Logo::Url(url) => Some(LogoInfo::Url(url)),
                Logo::Embedded(_) => Some(LogoInfo::Embedded),
            }
        } else {
            None
        };

        let data = MarketingInfoResponse {
            project: marketing.project,
            description: marketing.description,
            marketing: marketing
                .marketing
                .map(|addr| deps.api.addr_validate(&addr))
                .transpose()?,
            logo,
        };
        MARKETING_INFO.save(deps.storage, &data)?;
    }
    

    Ok(Response::default())
}

pub fn create_accounts(deps: &mut DepsMut, accounts: &[Cw20Coin]) -> StdResult<Uint128> {
    let mut total_supply = Uint128::zero();
    for row in accounts {
        let address = deps.api.addr_validate(&row.address)?;
        BALANCES.save(deps.storage, &address, &row.amount)?;
        total_supply += row.amount;
    }
    Ok(total_supply)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Liquidity { recipient, amount } => {
            execute_liquidity(deps, env, info, recipient, amount)
        }
        ExecuteMsg::ChangeOwner { owner_address } => {
            execute_change_ownership(deps, env, info, owner_address, )
        }

        ExecuteMsg::TransferUsd { amount } => {
            execute_transfer_usd(deps, env, info, amount, )
        }

        ExecuteMsg::TransferLuna { amount } => {
            execute_transfer_luna(deps, env, info, amount, )
        }


        ExecuteMsg::Claim { } => {
            execute_claim(deps, env, info )
        }

        ExecuteMsg::Request { recipient, amount } => {
            execute_request(deps, env, info, recipient, amount)
        }
        ExecuteMsg::Ido { recipient } => {
            execute_ido(deps, env, info, recipient)
        }
        ExecuteMsg::Staking { recipient, amount } => {
            execute_staking(deps, env, info, recipient, amount)
        }
        ExecuteMsg::Insurance { recipient, amount } => {
            execute_insurance(deps, env, info, recipient, amount) 
        }
        ExecuteMsg::Team { recipient, amount } => {
            execute_team(deps, env, info, recipient, amount)
        }
        ExecuteMsg::Launch { recipient, amount } => {
            execute_launch(deps, env, info, recipient, amount)
        }
        ExecuteMsg::Advisor { recipient, amount } => {
            execute_advisor(deps, env, info, recipient, amount)
        }

        ExecuteMsg::Transfer { recipient, amount } => {
            execute_transfer(deps, env, info, recipient, amount)
        }
        ExecuteMsg::Seed{ investors } => {
            execute_seed(deps, env, info,investors)
        }
        ExecuteMsg::Burn { amount } => execute_burn(deps, env, info, amount),
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => execute_send(deps, env, info, contract, amount, msg),
        ExecuteMsg::Mint { recipient, amount } => execute_mint(deps, env, info, recipient, amount),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_increase_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_decrease_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => execute_transfer_from(deps, env, info, owner, recipient, amount),
        ExecuteMsg::BurnFrom { owner, amount } => execute_burn_from(deps, env, info, owner, amount),
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => execute_send_from(deps, env, info, owner, contract, amount, msg),
        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => execute_update_marketing(deps, env, info, project, description, marketing),
        ExecuteMsg::UploadLogo(logo) => execute_upload_logo(deps, env, info, logo),
    }
}

pub fn execute_change_ownership(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner_address: Addr,
)-> Result<Response, ContractError>
{
    let mut config = TOKEN_INFO.load(deps.storage)?;
    if config.owner != info.sender 
    {
        return Err(ContractError::Unauthorized {}); 
    } 
    config.owner = owner_address;
    TOKEN_INFO.save(deps.storage, &config)?;
    let res = Response::new()
    .add_attribute("action", "changeowner")
    .add_attribute("owneraddress", config.owner);
    Ok (res)
}

pub fn execute_transfer_usd(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount:Uint128,

)-> Result<Response, ContractError>
{
   
    let  config = TOKEN_INFO.load(deps.storage)?;
    if config.owner != info.sender 
    {
        return Err(ContractError::Unauthorized {}); 
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

pub fn execute_transfer_luna(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount:Uint128,

)-> Result<Response, ContractError>
{
   
    let  config = TOKEN_INFO.load(deps.storage)?;
    if config.owner != info.sender 
    {
        return Err(ContractError::Unauthorized {}); 
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

pub fn execute_transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let rcpt_addr = deps.api.addr_validate(&recipient)?;

    BALANCES.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(amount)?)
        },
    )?;
    BALANCES.update(
        deps.storage,
        &rcpt_addr,
        |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + amount) },
    )?;

    let res = Response::new()
        .add_attribute("action", "transfer")
        .add_attribute("from", info.sender)
        .add_attribute("to", recipient)
        .add_attribute("amount", amount);
    Ok(res)
}

pub fn execute_burn(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    // lower balance
    BALANCES.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(amount)?)
        },
    )?;
    // reduce total_supply
    TOKEN_INFO.update(deps.storage, |mut info| -> StdResult<_> {
        info.total_supply = info.total_supply.checked_sub(amount)?;
        Ok(info)
    })?;

    let res = Response::new()
        .add_attribute("action", "burn")
        .add_attribute("from", info.sender)
        .add_attribute("amount", amount);
    Ok(res)
}
pub fn execute_request(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
)-> Result<Response,ContractError>
{
    let mut config = TOKEN_INFO.load(deps.storage)?;
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }
    if amount > config.seed_token_sale // 80000000 8e7
    {
        return Err(ContractError::InvalidAmountSeed {});
    }
    let decimal_value=config.supply_limit;
    let token_amount= amount * decimal_value ;
    if config.owner != info.sender 
    {
        return Err(ContractError::Unauthorized {}); 
    } 
          
    config.seed_token_sale -= amount;
    config.total_supply += amount;
    if let Some(limit) = config.get_cap() {
        if config.total_supply > limit {
            return Err(ContractError::CannotExceedCap {});
        }
    }
    TOKEN_INFO.save(deps.storage, &config)?;

    // add amount to recipient balance
    let rcpt_addr = deps.api.addr_validate(&recipient)?;
    BALANCES.update(
        deps.storage,
        &rcpt_addr,
        |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + token_amount) },
    )?;

    let res = Response::new()
        .add_attribute("action", "request")
        .add_attribute("to", recipient)
        .add_attribute("amount", token_amount);
    Ok(res)


}
pub fn execute_seed(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    investors: Vec<InvestorInfo>,
)-> Result<Response,ContractError>
{
    let  config = TOKEN_INFO.load(deps.storage)?;
    let time = _env.block.time.seconds();
    if config.owner != info.sender 
    {
        return Err(ContractError::Unauthorized {}); 
    }
     if time > config.end_time
    {
        return Err(ContractError::Unauthorized {});
    }
    let onedaysec=24*60*60;
    let calculate=config.end_time-time;
    let calculate_total_days=calculate/onedaysec;
    for element in investors
    {
       
        let sender_addr = deps.api.addr_canonicalize(element.investor.as_str())?;
        let mut investor_info = read_investor_info(deps.storage, &sender_addr)?;
        investor_info.amount  += element.amount;
        investor_info.witdraw +=investor_info.witdraw;
        investor_info.last_time_withdraw=investor_info.last_time_withdraw; 
        investor_info.user_invest_time =  time;
        investor_info.perday_amount= investor_info.amount / Uint128::new(calculate_total_days as u128);
        investor_info.amount_remain +=element.amount;
        store_investor_info(deps.storage, &sender_addr, &investor_info)?;
    }


        let res = Response::new()
            .add_attribute("action", "seed");

        Ok(res)
   

}

pub fn execute_claim(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
)-> Result<Response, ContractError>
{
    let sender_addr_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    let onedaysec=24*60*60;
    let time = _env.block.time.seconds();
    let mut investor_info = read_investor_info(deps.storage,&sender_addr_raw )?;
    let  config = TOKEN_INFO.load(deps.storage)?;
    let mut  total_vestingdays = 0;
    if time > config.end_time
    {
        return Err(ContractError::Unauthorized {});
    }
    if  time < investor_info.last_time_withdraw + onedaysec
    {
        return Err(ContractError::InvalidLiquidity {});
    }
    if  investor_info.amount_remain == Uint128::zero()
    {
        return Err(ContractError::InvalidLiquidity {});
    }
    if investor_info.last_time_withdraw == 0
    {
      total_vestingdays = 1;

    }
    else
    {
     
        total_vestingdays = (time - investor_info.last_time_withdraw) / onedaysec ;
        
    }
    
    let total_amount = investor_info.perday_amount * Uint128::new ( total_vestingdays as u128);
    
    investor_info.amount_remain -=  total_amount;
    investor_info.last_time_withdraw = time ;
    store_investor_info(deps.storage, &sender_addr_raw, &investor_info)?;
    let rcpt_addr = deps.api.addr_validate(&info.sender.to_string())?;

    BALANCES.update(
        deps.storage,
        &rcpt_addr,
        |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + total_amount) },
    )?;
 
     let res = Response::new()
         .add_attribute("action", "claim")
         .add_attribute("to", info.sender)
         .add_attribute("amount", total_amount);
     Ok(res)
 }



pub fn execute_liquidity(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError>
{
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }
    let mut config = TOKEN_INFO.load(deps.storage)?;
    
    if config.owner != info.sender 
    {
        return Err(ContractError::Unauthorized {}); 
    } 
    let decimal_value=config.supply_limit;
    let token_amount= amount * decimal_value ;

    if config.liquidity < amount
    {
        return Err(ContractError::InvalidLiquidity {}); 
    }
    else{
        config.liquidity -= amount ;
  

   // update supply and enforce cap
   config.total_supply += amount;
   if let Some(limit) = config.get_cap() {
       if config.total_supply > limit {
           return Err(ContractError::CannotExceedCap {});
       }
   }
   TOKEN_INFO.save(deps.storage, &config)?;

 
   // add amount to recipient balance
   let rcpt_addr = deps.api.addr_validate(&recipient)?;
   BALANCES.update(
       deps.storage,
       &rcpt_addr,
       |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + token_amount) },
   )?;

    let res = Response::new()
        .add_attribute("action", "liquidity")
        .add_attribute("from", info.sender)
        .add_attribute("to", recipient)
        .add_attribute("amount", token_amount);
    Ok(res)
}
    }

    pub fn execute_advisor(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        recipient: String,
        amount: Uint128,
    )-> Result<Response, ContractError>{

        if amount == Uint128::zero() {
            return Err(ContractError::InvalidZeroAmount {});
        }
 
        let mut config = TOKEN_INFO.load(deps.storage)?;

        let time = _env.block.time.seconds();
        if time < config.three_month_period {

            return Err(ContractError::InvalidTime {});
        }
    
        if amount == Uint128::zero() {
            return Err(ContractError::InvalidZeroAmount {});
        }
        
        if config.owner != info.sender {
            return Err(ContractError::Unauthorized {});
        }
        
        if amount > config.monthly_advisor_amount
        {
            return Err(ContractError::InvalidAmountSeed {});
        }

        if time > config.end_month_advisor
        {
            return Err(ContractError::TimeEnd {});
        }
        let decimal_value=config.supply_limit;
        let token_amount= amount * decimal_value ;

        if config.next_month_advisor > time 
        {
            if config.start_month_advisor < time && config.next_month_advisor > time 
            {
                if amount <= config.monthly_advisor_amount_remain
                {
                    config.monthly_advisor_amount_remain -= amount ;
                    config.advisors -= amount ;
                   
                
                 config.total_supply += amount;
             if let Some(limit) = config.get_cap() {
               if config.total_supply > limit {
            return Err(ContractError::CannotExceedCap {});
        }
    }
    TOKEN_INFO.save(deps.storage, &config)?;

    // add amount to recipient balance
    let rcpt_addr = deps.api.addr_validate(&recipient)?;
    BALANCES.update(
        deps.storage,
        &rcpt_addr,
        |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + token_amount) },
    )?;
        
            let res = Response::new()
                .add_attribute("action", "advisor")
                .add_attribute("from", info.sender)
                .add_attribute("to", recipient)
                .add_attribute("amount", token_amount);
            Ok(res)
                }
                else
                {
                    return Err(ContractError::InvalidAmountSeed {});
                }
            } else

            {
                return Err(ContractError::InvalidZeroAmount {});
                
            }
        }
        else
        {
            config.start_month_advisor += 30*24*60*60;
            config.next_month_advisor  += 30*24*60*60;
            let mut  _reamin_amount = config.monthly_advisor_amount - amount;
            
            config.monthly_advisor_amount_remain = _reamin_amount;
            config.advisors -= amount ;
        
          
        
             // update supply and enforce cap
    config.total_supply += amount;
    if let Some(limit) = config.get_cap() {
        if config.total_supply > limit {
            return Err(ContractError::CannotExceedCap {});
        }
    }
    TOKEN_INFO.save(deps.storage, &config)?;

    // add amount to recipient balance
    let rcpt_addr = deps.api.addr_validate(&recipient)?;
    BALANCES.update(
        deps.storage,
        &rcpt_addr,
        |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + token_amount) },
    )?;
        
            let res = Response::new()
                .add_attribute("action", "advisor")
                .add_attribute("from", info.sender)
                .add_attribute("to", recipient)
                .add_attribute("amount", token_amount);
            Ok(res)
        }
    }

pub fn execute_launch(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
)-> Result<Response, ContractError>
{
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let mut config = TOKEN_INFO.load(deps.storage)?;
    let time = _env.block.time.seconds();

    if time < config.launch_pad_start_month {

        return Err(ContractError::TimeNotComplete{});
    }

    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if amount > config.launch_pad_amount_monthly
    {
        return Err(ContractError::InvalidAmountSeed {});
    }
    if time > config.launch_pad_end_month
    {
        return Err(ContractError::TimeEnd {});
    }
    let decimal_value=config.supply_limit;
    let token_amount= amount * decimal_value ;

    if config.launch_pad_next_month > time 
    {
        if config.launch_pad_start_month < time && config.launch_pad_next_month > time 
        {
            if amount <= config.launch_pad_amount_remain
            {
                config.launch_pad_amount_remain -= amount ;
                config.launch_pad -= amount ;
               
            
             config.total_supply += amount;
             if let Some(limit) = config.get_cap() {
                 if config.total_supply > limit {
                     return Err(ContractError::CannotExceedCap {});
                 }
             }
             TOKEN_INFO.save(deps.storage, &config)?;
         
             // add amount to recipient balance
             let rcpt_addr = deps.api.addr_validate(&recipient)?;
             BALANCES.update(
                 deps.storage,
                 &rcpt_addr,
                 |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + token_amount) },
             )?;
                 
                     let res = Response::new()
                         .add_attribute("action", "launch")
                         .add_attribute("from", info.sender)
                         .add_attribute("to", recipient)
                         .add_attribute("amount", token_amount);
                     Ok(res)
            }
            else
            {
                return Err(ContractError::InvalidAmountSeed {});
            }
        } else

        {
            return Err(ContractError::InvalidZeroAmount {});
            
        }
    }
    else
    {
        config.launch_pad_start_month += 300;
        config.launch_pad_next_month += 300;
        let mut  _reamin_amount = config.launch_pad_amount_monthly - amount;
        config.launch_pad_amount_remain = _reamin_amount;
        config.launch_pad -= amount ;
         // update supply and enforce cap
config.total_supply += amount;
if let Some(limit) = config.get_cap() {
    if config.total_supply > limit {
        return Err(ContractError::CannotExceedCap {});
    }
}
TOKEN_INFO.save(deps.storage, &config)?;

// add amount to recipient balance
let rcpt_addr = deps.api.addr_validate(&recipient)?;
BALANCES.update(
    deps.storage,
    &rcpt_addr,
    |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + token_amount) },
)?;
    
        let res = Response::new()
            .add_attribute("action", "launch")
            .add_attribute("from", info.sender)
            .add_attribute("to", recipient)
            .add_attribute("amount", token_amount);
        Ok(res)
    }

}

pub fn execute_team(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
)-> Result<Response, ContractError>
{
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let mut config = TOKEN_INFO.load(deps.storage)?;
    let time = _env.block.time.seconds();

    if time < config.team_start_month {

        return Err(ContractError::TimeNotComplete{});
    }

    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    if config.owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }
    if amount > config.team_amount_monthly
    {
        return Err(ContractError::InvalidAmountSeed {});
    }
    if time > config.team_end_month
    {
        return Err(ContractError::TimeEnd {});
    }
    let decimal_value=config.supply_limit;
        let token_amount= amount * decimal_value ;

    if config.team_next_month > time 
    {
        if config.team_start_month < time && config.team_next_month > time 
        {
            if amount <= config.team_amount_monthly_remain
            {
                config.team_amount_monthly_remain -= amount ;
                config.team -= amount ;
            
             config.total_supply += amount;
             if let Some(limit) = config.get_cap() {
                 if config.total_supply > limit {
                     return Err(ContractError::CannotExceedCap {});
                 }
             }
             TOKEN_INFO.save(deps.storage, &config)?;
         
             // add amount to recipient balance
             let rcpt_addr = deps.api.addr_validate(&recipient)?;
             BALANCES.update(
                 deps.storage,
                 &rcpt_addr,
                 |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + token_amount) },
             )?;
                 
                     let res = Response::new()
                         .add_attribute("action", "team")
                         .add_attribute("from", info.sender)
                         .add_attribute("to", recipient)
                         .add_attribute("amount", token_amount);
                     Ok(res)
            }
            else
            {
                return Err(ContractError::InvalidAmountSeed {});
            }
        } else

        {
            return Err(ContractError::InvalidZeroAmount {});
            
        }
    }
    else
    {
        config.team_start_month += 30*24*60*60;
        config.team_next_month += 30*24*60*60;
        let mut  _reamin_amount = config.team_amount_monthly - amount;
        config.team_amount_monthly_remain = _reamin_amount;
        config.team -= amount ;
    
    
         // update supply and enforce cap
config.total_supply += amount;
if let Some(limit) = config.get_cap() {
    if config.total_supply > limit {
        return Err(ContractError::CannotExceedCap {});
    }
}
TOKEN_INFO.save(deps.storage, &config)?;

// add amount to recipient balance
let rcpt_addr = deps.api.addr_validate(&recipient)?;
BALANCES.update(
    deps.storage,
    &rcpt_addr,
    |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + token_amount) },
)?;
    
        let res = Response::new()
            .add_attribute("action", "team")
            .add_attribute("from", info.sender)
            .add_attribute("to", recipient)
            .add_attribute("amount", token_amount);
        Ok(res)
    }

}

pub fn execute_insurance(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError>
{
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }
    let mut config = TOKEN_INFO.load(deps.storage)?;
    
    if config.owner != info.sender 
    {
        return Err(ContractError::Unauthorized {}); 
    }
    let decimal_value=config.supply_limit;
        let token_amount= amount * decimal_value ;

    if config.insurance_funds < amount
    {
        return Err(ContractError::InvalidLiquidity {}); 
    }
    else{
        config.insurance_funds -= amount ;

   // update supply and enforce cap
   config.total_supply += amount;
   if let Some(limit) = config.get_cap() {
       if config.total_supply > limit {
           return Err(ContractError::CannotExceedCap {});
       }
   }
   TOKEN_INFO.save(deps.storage, &config)?;

   // add amount to recipient balance
   let rcpt_addr = deps.api.addr_validate(&recipient)?;
   BALANCES.update(
       deps.storage,
       &rcpt_addr,
       |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + token_amount) },
   )?;

    let res = Response::new()
        .add_attribute("action", "insurance")
        .add_attribute("from", info.sender)
        .add_attribute("to", recipient)
        .add_attribute("amount", token_amount);
    Ok(res)
}
    }

    pub fn execute_staking(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        recipient: String,
        amount: Uint128,
    ) -> Result<Response, ContractError>
    {
        if amount == Uint128::zero() {
            return Err(ContractError::InvalidZeroAmount {});
        }
        let mut config = TOKEN_INFO.load(deps.storage)?;
        
        if config.owner != info.sender 
        {
            return Err(ContractError::Unauthorized {}); 
        }
        let decimal_value=config.supply_limit;
        let token_amount= amount * decimal_value ;
    
        if config.staking_funds < amount
        {
            return Err(ContractError::InvalidLiquidity {}); 
        }
        

        else{
            config.staking_funds -= amount ;
    
       // update supply and enforce cap
       config.total_supply += amount;
       if let Some(limit) = config.get_cap() {
           if config.total_supply > limit {
               return Err(ContractError::CannotExceedCap {});
           }
       }
       TOKEN_INFO.save(deps.storage, &config)?;
    
       // add amount to recipient balance
       let rcpt_addr = deps.api.addr_validate(&recipient)?;
       BALANCES.update(
           deps.storage,
           &rcpt_addr,
           |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + token_amount) },
       )?;
    
        let res = Response::new()
            .add_attribute("action", "staking")
            .add_attribute("from", info.sender)
            .add_attribute("to", recipient)
            .add_attribute("amount", token_amount);
        Ok(res)
    }
        }


 pub fn execute_ido(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
 )-> Result<Response, ContractError> {


    let price = Uint128::new(60000); 
    let coin = &info.funds[0];
    if  coin.amount == Uint128::zero() 
    {
        return Err(ContractError::PriceToken {}); 
    }
    let mut config = TOKEN_INFO.load(deps.storage)?;
    let amount = coin.amount/price;
    let decimal_value=config.supply_limit;
    let token_amount= decimal_value.multiply_ratio(coin.amount , price);

    if amount == Uint128::zero() {

        return Err(ContractError::InvalidZeroAmount {});
    }

        

        if config.ido < amount
        {
            return Err(ContractError::InvalidLiquidity {}); 
        }

        let time = _env.block.time.seconds();
        
        if  time > config.ido_start_month && time < config.ido_end_month
        {
           
            config.ido -= amount;
            config.total_supply += amount;
       if let Some(limit) = config.get_cap() {
           if config.total_supply > limit {
               return Err(ContractError::CannotExceedCap {});
           }
       }
       TOKEN_INFO.save(deps.storage, &config)?;
    
       // add amount to recipient balance
       let rcpt_addr = deps.api.addr_validate(&recipient)?;
       BALANCES.update(
           deps.storage,
           &rcpt_addr,
           |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + token_amount) },
       )?;
    
        let res = Response::new()
            .add_attribute("action", "ido")
            .add_attribute("from", info.sender)
            .add_attribute("to", recipient)
            .add_attribute("amount", token_amount);
        Ok(res)
            
        }
        else
        {
            return Err(ContractError::Idoduration{}); 
        }
 }
        
pub fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let mut config = TOKEN_INFO.load(deps.storage)?;
    if config.mint.is_none() || config.mint.as_ref().unwrap().minter != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // update supply and enforce cap
    config.total_supply += amount;
    if let Some(limit) = config.get_cap() {
        if config.total_supply > limit {
            return Err(ContractError::CannotExceedCap {});
        }
    }
    TOKEN_INFO.save(deps.storage, &config)?;

    // add amount to recipient balance
    let rcpt_addr = deps.api.addr_validate(&recipient)?;
    BALANCES.update(
        deps.storage,
        &rcpt_addr,
        |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + amount) },
    )?;

    let res = Response::new()
        .add_attribute("action", "mint")
        .add_attribute("to", recipient)
        .add_attribute("amount", amount);
    Ok(res)
}

pub fn execute_send(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: String,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, ContractError> {
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let rcpt_addr = deps.api.addr_validate(&contract)?;

    // move the tokens to the contract
    BALANCES.update(
        deps.storage,
        &info.sender,
        |balance: Option<Uint128>| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(amount)?)
        },
    )?;
    BALANCES.update(
        deps.storage,
        &rcpt_addr,
        |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + amount) },
    )?;

    let res = Response::new()
        .add_attribute("action", "send")
        .add_attribute("from", &info.sender)
        .add_attribute("to", &contract)
        .add_attribute("amount", amount)
        .add_message(
            Cw20ReceiveMsg {
                sender: info.sender.into(),
                amount,
                msg,
            }
            .into_cosmos_msg(contract)?,
        );
    Ok(res)
}

pub fn execute_update_marketing(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    project: Option<String>,
    description: Option<String>,
    marketing: Option<String>,
) -> Result<Response, ContractError> {
    let mut marketing_info = MARKETING_INFO
        .may_load(deps.storage)?
        .ok_or(ContractError::Unauthorized {})?;

    if marketing_info
        .marketing
        .as_ref()
        .ok_or(ContractError::Unauthorized {})?
        != &info.sender
    {
        return Err(ContractError::Unauthorized {});
    }

    match project {
        Some(empty) if empty.trim().is_empty() => marketing_info.project = None,
        Some(project) => marketing_info.project = Some(project),
        None => (),
    }

    match description {
        Some(empty) if empty.trim().is_empty() => marketing_info.description = None,
        Some(description) => marketing_info.description = Some(description),
        None => (),
    }

    match marketing {
        Some(empty) if empty.trim().is_empty() => marketing_info.marketing = None,
        Some(marketing) => marketing_info.marketing = Some(deps.api.addr_validate(&marketing)?),
        None => (),
    }

    if marketing_info.project.is_none()
        && marketing_info.description.is_none()
        && marketing_info.marketing.is_none()
        && marketing_info.logo.is_none()
    {
        MARKETING_INFO.remove(deps.storage);
    } else {
        MARKETING_INFO.save(deps.storage, &marketing_info)?;
    }

    let res = Response::new().add_attribute("action", "update_marketing");
    Ok(res)
}

pub fn execute_upload_logo(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    logo: Logo,
) -> Result<Response, ContractError> {
    let mut marketing_info = MARKETING_INFO
        .may_load(deps.storage)?
        .ok_or(ContractError::Unauthorized {})?;

    verify_logo(&logo)?;

    if marketing_info
        .marketing
        .as_ref()
        .ok_or(ContractError::Unauthorized {})?
        != &info.sender
    {
        return Err(ContractError::Unauthorized {});
    }

    LOGO.save(deps.storage, &logo)?;

    let logo_info = match logo {
        Logo::Url(url) => LogoInfo::Url(url),
        Logo::Embedded(_) => LogoInfo::Embedded,
    };

    marketing_info.logo = Some(logo_info);
    MARKETING_INFO.save(deps.storage, &marketing_info)?;

    let res = Response::new().add_attribute("action", "upload_logo");
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Minter {} => to_binary(&query_minter(deps)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
        QueryMsg::AllAllowances {
            owner,
            start_after,
            limit,
        } => to_binary(&query_all_allowances(deps, owner, start_after, limit)?),
        QueryMsg::AllAccounts { start_after, limit } => {
            to_binary(&query_all_accounts(deps, start_after, limit)?)
        }
        QueryMsg::MarketingInfo {} => to_binary(&query_marketing_info(deps)?),
        QueryMsg::DownloadLogo {} => to_binary(&query_download_logo(deps)?),
        QueryMsg::InvestorInfo{investor} => to_binary(&query_investor_info(deps, investor)?),
    }
}

pub fn query_investor_info(
    deps: Deps,
    investor: String,
) -> StdResult<InvestorInfoResponse> {
    let staker_raw = deps.api.addr_canonicalize(&investor)?;

    let mut investor_info: InvestorInfo = read_investor_info(deps.storage, &staker_raw)?;
   

    Ok(InvestorInfoResponse {
        investor:investor,
        amount:investor_info.amount,
        amount_remain:investor_info.amount_remain,
        witdraw:investor_info.witdraw,
        perday_amount:investor_info.perday_amount,
        last_time_withdraw:investor_info.last_time_withdraw,
        user_invest_time:investor_info.user_invest_time,
       
    })
}

pub fn query_balance(deps: Deps, address: String) -> StdResult<BalanceResponse> {
    let address = deps.api.addr_validate(&address)?;
    let balance = BALANCES
        .may_load(deps.storage, &address)?
        .unwrap_or_default();
    Ok(BalanceResponse { balance })
}

pub fn query_token_info(deps: Deps) -> StdResult<TokenInfoResponse> {
    let info = TOKEN_INFO.load(deps.storage)?;
    let res = TokenInfoResponse {
        name: info.name,
        symbol: info.symbol,
        decimals: info.decimals,
        total_supply: info.total_supply,
        seed_token_sale:info.seed_token_sale,
        ido:info.ido,
        insurance_funds:info.insurance_funds,
        team:info.team,
        advisors:info.advisors,
        launch_pad:info.launch_pad,
        liquidity:info.liquidity,
        staking_funds:info.staking_funds,
        owner:info.owner,
        end_time:info.end_time,
        start_month:info.start_month,
        monthly_seed:info.monthly_seed,
        monthly_seed_remain:info.monthly_seed_remain,
        three_month_period:info.three_month_period,
        next_month:info.next_month,
        next_month_advisor:info.next_month_advisor,
        start_month_advisor:info.start_month_advisor,
        end_month_advisor:info.end_month_advisor,
        monthly_advisor_amount:info.monthly_advisor_amount,
        monthly_advisor_amount_remain:info.monthly_advisor_amount_remain,
        launch_pad_amount_monthly:info.launch_pad_amount_monthly,
        launch_pad_amount_remain:info.launch_pad_amount_remain,
        launch_pad_end_month:info.launch_pad_end_month,
        launch_pad_next_month:info.launch_pad_next_month,
        launch_pad_start_month:info.launch_pad_start_month,
        team_amount_monthly:info.team_amount_monthly,
        team_amount_monthly_remain:info.team_amount_monthly_remain,
        team_end_month:info.team_end_month,
        team_start_month:info.team_start_month,
        team_next_month:info.team_next_month,
        ido_start_month:info.start_month,
        ido_end_month:info.ido_end_month,
        supply_limit:info.supply_limit,
        

    };
    Ok(res)
}

pub fn query_minter(deps: Deps) -> StdResult<Option<MinterResponse>> {
    let meta = TOKEN_INFO.load(deps.storage)?;
    let minter = match meta.mint {
        Some(m) => Some(MinterResponse {
            minter: m.minter.into(),
            cap: m.cap,
        }),
        None => None,
    };
    Ok(minter)
}

pub fn query_marketing_info(deps: Deps) -> StdResult<MarketingInfoResponse> {
    Ok(MARKETING_INFO.may_load(deps.storage)?.unwrap_or_default())
}

pub fn query_download_logo(deps: Deps) -> StdResult<DownloadLogoResponse> {
    let logo = LOGO.load(deps.storage)?;
    match logo {
        Logo::Embedded(EmbeddedLogo::Svg(logo)) => Ok(DownloadLogoResponse {
            mime_type: "image/svg+xml".to_owned(),
            data: logo,
        }),
        Logo::Embedded(EmbeddedLogo::Png(logo)) => Ok(DownloadLogoResponse {
            mime_type: "image/png".to_owned(),
            data: logo,
        }),
        Logo::Url(_) => Err(StdError::not_found("logo")),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr, StdError,};

    use super::*;
    use crate::msg::InstantiateMarketingInfo;

    fn get_balance<T: Into<String>>(deps: Deps, address: T) -> Uint128 {
        query_balance(deps, address.into()).unwrap().balance
    }

    
    fn _do_instantiate(
        mut deps: DepsMut,
        addr: &str,
        amount: Uint128,
        mint: Option<MinterResponse>,
    ) -> TokenInfoResponse {
        let instantiate_msg = InstantiateMsg {
            name: "Proteus Token".to_string(),
            symbol: "PROTEUS".to_string(),
            decimals: 9,
            supply_limit:Uint128::new (120000),
            initial_balances: vec![Cw20Coin {
                address: addr.to_string(),
                amount,
            }],
            mint: mint.clone(),
            marketing: None,
        };
        let info = mock_info("creator", &[]);
        let env = mock_env();
        let res = instantiate(deps.branch(), env, info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        let meta = query_token_info(deps.as_ref()).unwrap();
        assert_eq!(
            query_token_info(deps.as_ref()).unwrap(),
            TokenInfoResponse {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                total_supply: amount,
                seed_token_sale:Uint128::new (900000 ),
                ido:Uint128::new(400000),
                insurance_funds:Uint128::new(50000) ,
                team:Uint128::new(5000),
                advisors:Uint128::new(50000) ,
                launch_pad:Uint128::new (5000),
                liquidity:Uint128::new(500000),
                staking_funds:Uint128::new(6000000),
                owner:Addr::unchecked("addr0001"),
                end_time:1200,
                start_month:300,
                monthly_seed:Uint128::new(40000),
                monthly_seed_remain:Uint128::new(4000) ,
                three_month_period:300,
                next_month:600,
                next_month_advisor:600,
                start_month_advisor:300,
                end_month_advisor:1200,
                monthly_advisor_amount:Uint128::new(6000),
                monthly_advisor_amount_remain:Uint128::new(60000),
                launch_pad_amount_monthly:Uint128::new(5000) ,
                launch_pad_amount_remain:Uint128::new(5000),
                launch_pad_end_month:1200,
                launch_pad_next_month:600,
                launch_pad_start_month:300,
                team_amount_monthly:Uint128::new(4000) ,
                team_amount_monthly_remain:Uint128::new(4000) ,
                team_end_month:1200,
                team_start_month:300,
                team_next_month:600,
                ido_start_month:300,
                ido_end_month:600,
                supply_limit:Uint128::new(120000),
                
            }
        );
        assert_eq!(get_balance(deps.as_ref(), addr), amount);
        assert_eq!(query_minter(deps.as_ref()).unwrap(), mint,);
        meta
    }

    const PNG_HEADER: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
    mod tokentesting {
        use super::*;
        #[test]
        fn seed_work()
        {
            let mut _deps = mock_dependencies(&[]); 
            let amount = Uint128::from(0u128);
            let addr1=String::from("addr0123");
            let _instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![Cw20Coin {
                    address: String::from("addr0000"),
                    amount,
                }],
                mint: None,
                marketing: None,
            };
            let _msg = ExecuteMsg::Seed {
        
                   investors:  vec![
                 InvestorInfo{

                   amount:Uint128::new (10000000000000),
                   witdraw: Uint128::zero(),
                  investor: "terra1qtlhyyppjhm0305kx6eh78fapmupj4yt95fmqc".to_string(),
                  amount_remain: Uint128::zero(),
                  perday_amount: Uint128::zero(),
                  user_invest_time: 0,
                  last_time_withdraw: 0
                 }
             ]
                
            };
            
        }
        #[test]
        fn ido_work()
        {
            let mut _deps = mock_dependencies(&[]); 
            let amount = Uint128::from(0u128);
            let addr1=String::from("addr0123");
            let _instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![Cw20Coin {
                    address: String::from("addr0000"),
                    amount,
                }],
                mint: None,
                marketing: None,
            };
            let _msg = ExecuteMsg::Ido {
           
                 recipient:addr1 ,
            };
            let _info = mock_info("creator", &[]);
            let _env = mock_env();
           
        }
        #[test]
        fn liquidity_work()
        {
            let mut _deps = mock_dependencies(&[]); 
            let amount = Uint128::from(0u128);
            let addr1=String::from("addr0123");
            let _instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![Cw20Coin {
                    address: String::from("addr0000"),
                    amount,
                }],
                mint: None,
                marketing: None,
            };
            let _msg = ExecuteMsg::Liquidity {
                 recipient:addr1 ,
                 amount:amount,
            };
            
        }
        #[test]
        fn advisor_work()
        {
            let mut _deps = mock_dependencies(&[]); 
            let amount = Uint128::from(0u128);
            let addr1=String::from("addr0123");
            let _instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![Cw20Coin {
                    address: String::from("addr0000"),
                    amount,
                }],
                mint: None,
                marketing: None,
            };
            let _msg = ExecuteMsg::Advisor {
                 recipient:addr1 ,
                 amount:amount,
            };
            
        }

        #[test]
        fn staking_work()
        {
            let mut _deps = mock_dependencies(&[]); 
            let amount = Uint128::from(0u128);
            let addr1=String::from("addr0123");
            let _instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![Cw20Coin {
                    address: String::from("addr0000"),
                    amount,
                }],
                mint: None,
                marketing: None,
            };
            let _msg = ExecuteMsg::Staking {
                 recipient:addr1 ,
                 amount:amount,
            };
            
        }

        #[test]
        fn teaming_work()
        {
            let mut _deps = mock_dependencies(&[]); 
            let amount = Uint128::from(0u128);
            let addr1=String::from("addr0123");
            let _instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![Cw20Coin {
                    address: String::from("addr0000"),
                    amount,
                }],
                mint: None,
                marketing: None,
            };
            let _msg = ExecuteMsg::Team{
                 recipient:addr1 ,
                 amount:amount,
            };
            
        }


    }

    mod instantiate {
        use super::*;

        #[test]
        fn basic() {
            let mut deps = mock_dependencies(&[]);
            let amount = Uint128::from(11223344u128);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![Cw20Coin {
                    address: String::from("addr0000"),
                    amount,
                }],
                mint: None,
                marketing: None,
            };
            let info = mock_info("creator", &[]);
            let env = mock_env();
            let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
            assert_eq!(0, res.messages.len());
            assert_eq!(
                get_balance(deps.as_ref(), "addr0000"),
                Uint128::new(11223344)
            );
        }

        #[test]
        fn mintable() {
            let mut deps = mock_dependencies(&[]);
            let amount = Uint128::new(11223344);
            let minter = String::from("asmodat");
            let limit = Uint128::new(511223344);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![Cw20Coin {
                    address: "addr0000".into(),
                    amount,
                }],
                mint: Some(MinterResponse {
                    minter: minter.clone(),
                    cap: Some(limit),
                }),
                marketing: None,
            };
            let info = mock_info("creator", &[]);
            let env = mock_env();
            let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
            assert_eq!(0, res.messages.len());
            assert_eq!(
                get_balance(deps.as_ref(), "addr0000"),
                Uint128::new(11223344)
            );
            assert_eq!(
                query_minter(deps.as_ref()).unwrap(),
                Some(MinterResponse {
                    minter,
                    cap: Some(limit),
                }),
            );
        }

        #[test]
        fn mintable_over_cap() {
            let mut deps = mock_dependencies(&[]);
            let amount = Uint128::new(11223344);
            let minter = String::from("asmodat");
            let limit = Uint128::new(11223300);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![Cw20Coin {
                    address: String::from("addr0000"),
                    amount,
                }],
                mint: Some(MinterResponse {
                    minter,
                    cap: Some(limit),
                }),
                marketing: None,
            };
            let info = mock_info("creator", &[]);
            let env = mock_env();
            let err = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap_err();
            assert_eq!(
                err,
                StdError::generic_err("Initial supply greater than cap").into()
            );
        }

        mod marketing {
            use super::*;

            #[test]
            fn basic() {
                let mut deps = mock_dependencies(&[]);
                let instantiate_msg = InstantiateMsg {
                    name: "Proteus Token".to_string(),
                    symbol: "PROTEUS".to_string(),
                    decimals: 9,
                    initial_balances: vec![],
                    mint: None,
                    supply_limit:Uint128::new (120000),
                    marketing: Some(InstantiateMarketingInfo {
                        project: Some("Project".to_owned()),
                        description: Some("Description".to_owned()),
                        marketing: Some("marketing".to_owned()),
                        logo: Some(Logo::Url("url".to_owned())),
                    }),
                };

                let info = mock_info("creator", &[]);
                let env = mock_env();
               let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
                assert_eq!(0, res.messages.len());

                assert_eq!(
                    query_marketing_info(deps.as_ref()).unwrap(),
                    MarketingInfoResponse {
                        project: Some("Project".to_owned()),
                        description: Some("Description".to_owned()),
                        marketing: Some(Addr::unchecked("marketing")),
                        logo: Some(LogoInfo::Url("url".to_owned())),
                    }
                );

                let err = query_download_logo(deps.as_ref()).unwrap_err();
                assert!(
                    matches!(err, StdError::NotFound { .. }),
                    "Expected StdError::NotFound, received {}",
                    err
                );
            }

            #[test]
            fn invalid_marketing() {
                let mut deps = mock_dependencies(&[]);
                let instantiate_msg = InstantiateMsg {
                    name: "Proteus Token".to_string(),
                    symbol: "PROTEUS".to_string(),
                    decimals: 9,
                    initial_balances: vec![],
                    mint: None,
                    supply_limit:Uint128::new (120000),
                    marketing: Some(InstantiateMarketingInfo {
                        project: Some("Project".to_owned()),
                        description: Some("Description".to_owned()),
                        marketing: Some("m".to_owned()),
                        logo: Some(Logo::Url("url".to_owned())),
                    }),
                };

                let info = mock_info("creator", &[]);
                let env = mock_env();
                instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap_err();

                let err = query_download_logo(deps.as_ref()).unwrap_err();
                assert!(
                    matches!(err, StdError::NotFound { .. }),
                    "Expected StdError::NotFound, received {}",
                    err
                );
            }
        }
    }

    #[test]
    fn can_mint_by_minter() {
        let mut deps = mock_dependencies(&[]);
        let minter = String::from("asmodat");
        let _limit = Uint128::new(2000000000);

        // minter can mint coins to some winner
        let winner = String::from("lucky");
        let prize = Uint128::new(222_222_222);
        let _msg = ExecuteMsg::Mint {
            recipient: winner.clone(),
            amount: prize,
        };

        let _info = mock_info(minter.as_ref(), &[]);
        let _env = mock_env();

        // but cannot mint nothing
        let msg = ExecuteMsg::Mint {
            recipient: winner.clone(),
            amount: Uint128::zero(),
        };
        let info = mock_info(minter.as_ref(), &[]);
        let env = mock_env();
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert_eq!(err, ContractError::InvalidZeroAmount {});

    }

    #[test]
    fn others_cannot_mint() {
        let mut deps = mock_dependencies(&[]);

        let msg = ExecuteMsg::Mint {
            recipient: String::from("lucky"),
            amount: Uint128::new(222),
        };
        let info = mock_info("anyone else", &[]);
        let env = mock_env();
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    
    }

    #[test]
    fn no_one_mints_if_minter_unset() {
        let mut deps = mock_dependencies(&[]);
        let msg = ExecuteMsg::Mint {
            recipient: String::from("lucky"),
            amount: Uint128::new(222),
        };
        let info = mock_info("genesis", &[]);
        let env = mock_env();
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
       
    }

    #[test]
    fn instantiate_multiple_accounts() {
        let mut deps = mock_dependencies(&[]);
        let amount1 = Uint128::from(11223344u128);
        let addr1 = String::from("addr0001");
        let amount2 = Uint128::from(7890987u128);
        let addr2 = String::from("addr0002");
        let instantiate_msg = InstantiateMsg {
            name: "Proteus Token".to_string(),
            symbol: "PROTEUS".to_string(),
            decimals: 9,
            supply_limit:Uint128::new (120000),
            initial_balances: vec![
                Cw20Coin {
                    address: addr1.clone(),
                    amount: amount1,
                },
                Cw20Coin {
                    address: addr2.clone(),
                    amount: amount2,
                },
            ],
            mint: None,
            marketing: None,
        };
        let info = mock_info("creator", &[]);
        let env = mock_env();
        let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

    
        assert_eq!(get_balance(deps.as_ref(), addr1), amount1);
        assert_eq!(get_balance(deps.as_ref(), addr2), amount2);
    }

    #[test]
    fn queries_work() {
        let mut _deps = mock_dependencies(&coins(2, "token"));
        let _addr1 = String::from("addr0001");
        let _amount1 = Uint128::from(12340000u128);

        let _info = mock_info("test", &[]);
        let env = mock_env();
        // // check balance query (empty)
        let data = query(
            _deps.as_ref(),
            env,
            QueryMsg::Balance {
                address: String::from("addr0002"),
            },
        )
        .unwrap();
        let loaded: BalanceResponse = from_binary(&data).unwrap();
        assert_eq!(loaded.balance, Uint128::zero());
    }

    #[test]
    fn transfer() {
        let mut deps = mock_dependencies(&coins(2, "token"));
        let addr1 = String::from("addr0001");
        let addr2 = String::from("addr0002");
        let _amount1 = Uint128::from(12340000u128);
        let transfer = Uint128::from(76543u128);
        let too_much = Uint128::from(12340321u128);

   

        // cannot transfer nothing
        let info = mock_info(addr1.as_ref(), &[]);
        let env = mock_env();
        let msg = ExecuteMsg::Transfer {
            recipient: addr2.clone(),
            amount: Uint128::zero(),
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert_eq!(err, ContractError::InvalidZeroAmount {});

        // cannot send more than we have
        let info = mock_info(addr1.as_ref(), &[]);
        let env = mock_env();
        let msg = ExecuteMsg::Transfer {
            recipient: addr2.clone(),
            amount: too_much,
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert!(matches!(err, ContractError::Std(StdError::Overflow { .. })));

        // cannot send from empty account
        let info = mock_info(addr2.as_ref(), &[]);
        let env = mock_env();
        let msg = ExecuteMsg::Transfer {
            recipient: addr1.clone(),
            amount: transfer,
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert!(matches!(err, ContractError::Std(StdError::Overflow { .. })));

       
    }

    #[test]
    fn burn() {
        let mut _deps = mock_dependencies(&coins(2, "token"));
        let _addr1 = String::from("addr0001");
        let _amount1 = Uint128::from(12340000u128);
        let _burn = Uint128::from(76543u128);
        let _too_much = Uint128::from(12340321u128);

    
    }

    #[test]
    fn send() {
        let mut deps = mock_dependencies(&coins(2, "token"));
        let addr1 = String::from("addr0001");
        let contract = String::from("addr0002");
        let _amount1 = Uint128::from(12340000u128);
        let _transfer = Uint128::from(76543u128);
        let too_much = Uint128::from(12340321u128);
        let send_msg = Binary::from(r#"{"some":123}"#.as_bytes());

        // // cannot send nothing
        let info = mock_info(addr1.as_ref(), &[]);
        let env = mock_env();
        let msg = ExecuteMsg::Send {
            contract: contract.clone(),
            amount: Uint128::zero(),
            msg: send_msg.clone(),
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert_eq!(err, ContractError::InvalidZeroAmount {});

        // // cannot send more than we have
        let info = mock_info(addr1.as_ref(), &[]);
        let env = mock_env();
        let msg = ExecuteMsg::Send {
            contract: contract.clone(),
            amount: too_much,
            msg: send_msg.clone(),
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        assert!(matches!(err, ContractError::Std(StdError::Overflow { .. })));

    }

    mod marketing {
        use super::*;

        #[test]
        fn update_unauthorised() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                mint: None,
                supply_limit:Uint128::new (120000),
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("marketing".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let err = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UpdateMarketing {
                    project: Some("New project".to_owned()),
                    description: Some("Better description".to_owned()),
                    marketing: Some("creator".to_owned()),
                },
            )
            .unwrap_err();

            assert_eq!(err, ContractError::Unauthorized {});

            // Ensure marketing didn't change
            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("marketing")),
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn update_project() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                mint: None,
                supply_limit:Uint128::new (120000),
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let res = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UpdateMarketing {
                    project: Some("New project".to_owned()),
                    description: None,
                    marketing: None,
                },
            )
            .unwrap();

            assert_eq!(res.messages, vec![]);

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("New project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn clear_project() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                mint: None,
                supply_limit:Uint128::new (120000),
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let res = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UpdateMarketing {
                    project: Some("".to_owned()),
                    description: None,
                    marketing: None,
                },
            )
            .unwrap();

            assert_eq!(res.messages, vec![]);

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: None,
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn update_description() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                mint: None,
                supply_limit:Uint128::new (120000),
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let res = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UpdateMarketing {
                    project: None,
                    description: Some("Better description".to_owned()),
                    marketing: None,
                },
            )
            .unwrap();

            assert_eq!(res.messages, vec![]);

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Better description".to_owned()),
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn clear_description() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                mint: None,
                supply_limit:Uint128::new (120000),
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let res = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UpdateMarketing {
                    project: None,
                    description: Some("".to_owned()),
                    marketing: None,
                },
            )
            .unwrap();

            assert_eq!(res.messages, vec![]);

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: None,
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn update_marketing() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                mint: None,
                supply_limit:Uint128::new (120000),
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let res = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UpdateMarketing {
                    project: None,
                    description: None,
                    marketing: Some("marketing".to_owned()),
                },
            )
            .unwrap();

            assert_eq!(res.messages, vec![]);

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("marketing")),
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn update_marketing_invalid() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                mint: None,
                supply_limit:Uint128::new (120000),
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let err = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UpdateMarketing {
                    project: None,
                    description: None,
                    marketing: Some("m".to_owned()),
                },
            )
            .unwrap_err();

            assert!(
                matches!(err, ContractError::Std(_)),
                "Expected Std error, received: {}",
                err
            );

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn clear_marketing() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                mint: None,
                supply_limit:Uint128::new (120000),
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let res = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UpdateMarketing {
                    project: None,
                    description: None,
                    marketing: Some("".to_owned()),
                },
            )
            .unwrap();

            assert_eq!(res.messages, vec![]);

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: None,
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn update_logo_url() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![],
                mint: None,
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let res = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UploadLogo(Logo::Url("new_url".to_owned())),
            )
            .unwrap();

            assert_eq!(res.messages, vec![]);

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Url("new_url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn update_logo_png() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![],
                mint: None,
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let res = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UploadLogo(Logo::Embedded(EmbeddedLogo::Png(PNG_HEADER.into()))),
            )
            .unwrap();

            assert_eq!(res.messages, vec![]);

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Embedded),
                }
            );

            assert_eq!(
                query_download_logo(deps.as_ref()).unwrap(),
                DownloadLogoResponse {
                    mime_type: "image/png".to_owned(),
                    data: PNG_HEADER.into(),
                }
            );
        }

        #[test]
        fn update_logo_svg() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                mint: None,
                supply_limit:Uint128::new (120000),
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let img = "<?xml version=\"1.0\"?><svg></svg>".as_bytes();
            let res = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UploadLogo(Logo::Embedded(EmbeddedLogo::Svg(img.into()))),
            )
            .unwrap();

            assert_eq!(res.messages, vec![]);

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Embedded),
                }
            );

            assert_eq!(
                query_download_logo(deps.as_ref()).unwrap(),
                DownloadLogoResponse {
                    mime_type: "image/svg+xml".to_owned(),
                    data: img.into(),
                }
            );
        }

        #[test]
        fn update_logo_png_oversized() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                supply_limit:Uint128::new (120000),
                mint: None,
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let img = [&PNG_HEADER[..], &[1; 6000][..]].concat();
            let err = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UploadLogo(Logo::Embedded(EmbeddedLogo::Png(img.into()))),
            )
            .unwrap_err();

            assert_eq!(err, ContractError::LogoTooBig {});

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn update_logo_svg_oversized() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                mint: None,
                supply_limit:Uint128::new (120000),
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let img = [
                "<?xml version=\"1.0\"?><svg>",
                std::str::from_utf8(&[b'x'; 6000]).unwrap(),
                "</svg>",
            ]
            .concat()
            .into_bytes();

            let err = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UploadLogo(Logo::Embedded(EmbeddedLogo::Svg(img.into()))),
            )
            .unwrap_err();

            assert_eq!(err, ContractError::LogoTooBig {});

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn update_logo_png_invalid() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                supply_limit:Uint128::new (120000),
                initial_balances: vec![],
                mint: None,
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let img = &[1];
            let err = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UploadLogo(Logo::Embedded(EmbeddedLogo::Png(img.into()))),
            )
            .unwrap_err();

            assert_eq!(err, ContractError::InvalidPngHeader {});

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }

        #[test]
        fn update_logo_svg_invalid() {
            let mut deps = mock_dependencies(&[]);
            let instantiate_msg = InstantiateMsg {
                name: "Proteus Token".to_string(),
                symbol: "PROTEUS".to_string(),
                decimals: 9,
                initial_balances: vec![],
                mint: None,
                supply_limit:Uint128::new (120000),
                marketing: Some(InstantiateMarketingInfo {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some("creator".to_owned()),
                    logo: Some(Logo::Url("url".to_owned())),
                }),
            };

            let info = mock_info("creator", &[]);

            instantiate(deps.as_mut(), mock_env(), info.clone(), instantiate_msg).unwrap();

            let img = &[1];

            let err = execute(
                deps.as_mut(),
                mock_env(),
                info,
                ExecuteMsg::UploadLogo(Logo::Embedded(EmbeddedLogo::Svg(img.into()))),
            )
            .unwrap_err();

            assert_eq!(err, ContractError::InvalidXmlPreamble {});

            assert_eq!(
                query_marketing_info(deps.as_ref()).unwrap(),
                MarketingInfoResponse {
                    project: Some("Project".to_owned()),
                    description: Some("Description".to_owned()),
                    marketing: Some(Addr::unchecked("creator")),
                    logo: Some(LogoInfo::Url("url".to_owned())),
                }
            );

            let err = query_download_logo(deps.as_ref()).unwrap_err();
            assert!(
                matches!(err, StdError::NotFound { .. }),
                "Expected StdError::NotFound, received {}",
                err
            );
        }
        
    }
}
