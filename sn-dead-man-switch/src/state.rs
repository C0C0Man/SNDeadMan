use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use cosmwasm_std::{Addr, StdError, StdResult, Storage};
use secret_toolkit::serialization::{Bincode2, Serde}; // Import Serde trait
use secret_toolkit::storage::Keymap;
use crate::msg::AccountResponse;
use crate::contract::ContractError;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Account {
    pub address: Addr,
    pub balance: u128,
    pub password_hash: Option<Vec<u8>>,
}

// Constants for storage keys (using a more explicit naming convention)
const ACCOUNTS_KEY: &[u8] = b"accounts";

// Keymap for storing multiple accounts
pub const ACCOUNTS: Keymap<Addr, Account> = Keymap::new(ACCOUNTS_KEY);


// Returns StdResult<()> resulting from saving an item to storage
pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {
    storage.set(key, &Bincode2::serialize(value)?);
    Ok(())
}

// Removes an item from storage
pub fn remove<S: Storage>(storage: &mut S, key: &[u8]) {
    storage.remove(key);
}

// Returns StdResult<T> from retrieving the item with the specified key. Returns a
// StdError::NotFound if there is no item with that key
pub fn load<T: DeserializeOwned, S: Storage>(storage: &S, key: &[u8]) -> StdResult<T> {
    Bincode2::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(std::any::type_name::<T>()))?,
    )
}

// Function to store an account
pub fn store_account(storage: &mut dyn Storage, account: &Account, address: &Addr) -> StdResult<()> {
    ACCOUNTS.insert(storage, address, account) // Use Addr directly as the key
}



// Function to load an account by address
pub fn load_account(storage: &dyn Storage, address: &Addr) -> StdResult<Account> {
    ACCOUNTS.get(storage, address).ok_or(StdError::not_found("Account"))
}


// Update Balance
pub fn update_balance(
    storage: &mut dyn Storage,
    address: &Addr,
    amount: u128,
) -> Result<(), ContractError> {
    // Attempt to load the account; if not found, return an error
    let mut account = load_account(storage, address)?;

    if account.balance < amount {
        return Err(ContractError::InsufficientFunds {});
    }
    account.balance -= amount;
    store_account(storage, &account, &address)?; // Store the updated account back in the map

    Ok(())
}

// Set Password Hash
pub fn set_password_hash(
    storage: &mut dyn Storage,
    address: &Addr,
    password_hash: Vec<u8>,
) -> StdResult<()> {

    // Load the account
    let mut account = load_account(storage, address)?;

    // Update the password hash
    account.password_hash = Some(password_hash);

    // Store the updated account
    store_account(storage, &account, &address)?;

    Ok(())
}

// Validate Password
pub fn validate_password(
    storage: &dyn Storage,
    address: &Addr,
    password_attempt: &[u8],
) -> StdResult<bool> {
    let account = load_account(storage, address)?;
    if let Some(stored_hash) = account.password_hash {
        Ok(stored_hash == password_attempt)
    } else {
        Err(StdError::generic_err("No password set for this account"))
    }
}


// Get Balance
pub fn get_balance(storage: &dyn Storage, address: &Addr) -> StdResult<AccountResponse> {
    let account = load_account(storage, address)?;
    Ok(AccountResponse {
        address: account.address,
        balance: account.balance,
    })
}



