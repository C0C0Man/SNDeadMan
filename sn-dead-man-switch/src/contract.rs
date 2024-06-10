use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, StdError
};

use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Account, store_account, load_account, get_balance};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Account not found for address: {0}")]
    AccountNotFound(Addr),

    #[error("Account already exists")] 
    AccountAlreadyExists {},
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {

    Ok(Response::default())
}

// 2. execute
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InitWallet {  } => {
            execute_init_wallet(deps, info)
        },
    }
}


// Execute init wallet
pub fn execute_init_wallet(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {

let address = Addr::unchecked(info.sender.to_string());

    // Create a new account with 0 balance
    let account = Account {
        address: address.clone(),
        balance: 0,
    };

    // Store the account in state 
    store_account(deps.storage, &account)?; 

    Ok(Response::new()
        .add_attribute("action", "init_wallet")
        .add_attribute("address", address))
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance { address } => {
            let validated_address = deps.api.addr_validate(&address)?; // Validate address
            to_binary(&get_balance(deps, &validated_address, "uscrt".to_string())?) 
        }
    }
}

