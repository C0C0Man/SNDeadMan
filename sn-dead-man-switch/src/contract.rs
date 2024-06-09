use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Account, store_account, load_account, get_balance, validate_password };
use secret_toolkit::crypto::sha_256;

use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Account already exists")] 
    AccountAlreadyExists {},
}

// 1. instantiate 
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
        ExecuteMsg::InitWallet { address, password } => {
            execute_init_wallet(deps, info, address, password)
        },
        ExecuteMsg::SetPassword { 
            current_password, 
            new_password 
        } => execute_set_password(deps, info, current_password, new_password),
       
    }
}

// Execute init wallet
pub fn execute_init_wallet(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
    password: Option<String>,
) -> Result<Response, ContractError> {
    
    // Validate the address
    let validated_address = deps.api.addr_validate(&address)?; 

    // Ensure that the sender is trying to create their own wallet 
    if info.sender != validated_address{
        return Err(ContractError::Unauthorized {});
    }

    // Check if an account with the same address already exists
    if load_account(deps.storage, &validated_address).is_ok() {
        return Err(ContractError::AccountAlreadyExists {});
    }

    // Create a new account with 0 balance
    let mut account = Account {
        address: info.sender,
        balance: 0,
        password_hash: None,
    };

    // Optionally set the password hash if provided
    if let Some(password) = password {
        let password_hash = sha_256(password.as_bytes());
        account.password_hash = Some(password_hash.into());
    }

    // Store the account in state 
    store_account(deps.storage, &account, &validated_address)?; // <-- Pass validated_address

    Ok(Response::new()
        .add_attribute("action", "init_wallet")
        .add_attribute("address", address))
}

// Set Password Hash
pub fn execute_set_password(
    deps: DepsMut,
    info: MessageInfo,
    current_password: Option<String>,
    new_password: String,
) -> Result<Response, ContractError> {
    let validated_address = deps.api.addr_validate(&info.sender.clone().into_string())?;

    let mut account = load_account(deps.storage, &info.sender)?;

    // if the user is trying to set up their password for the first time
    if current_password.is_none() {
        account.password_hash = Some(sha_256(new_password.as_bytes()).into());
    // if the user is trying to update their password
    } else if let Some(current_password) = current_password {
        // Hash the passwords
        let current_password_hash = sha_256(current_password.as_bytes());
        let new_password_hash = sha_256(new_password.as_bytes());

        if !validate_password(deps.storage, &account.address, &current_password_hash)? {
            return Err(ContractError::Unauthorized {});
        }

        account.password_hash = Some(new_password_hash.into());
    }

    store_account(deps.storage, &account, &info.sender)?; // Pass in &info.sender
    Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance { address } => to_binary(&get_balance(deps.storage, &deps.api.addr_validate(&address)?)?), 
    }
}
