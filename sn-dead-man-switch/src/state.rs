use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use cosmwasm_std::{Addr, StdError, StdResult, Storage, Deps};
use secret_toolkit::serialization::Bincode2;
use secret_toolkit::serialization::Serde;
use secret_toolkit::storage::Keymap;
use crate::msg::AccountResponse;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Account {
    pub address: Addr,
    pub balance: u128,
}

// Constants for storage keys 
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
pub fn store_account(storage: &mut dyn Storage, account: &Account) -> StdResult<()> {
    ACCOUNTS.insert(storage, &account.address, account)
}


// Function to load an account by address
pub fn load_account(storage: &dyn Storage, address: &Addr) -> StdResult<Account> {
    ACCOUNTS.get(storage, address).ok_or(StdError::not_found("Account"))
}


pub fn get_balance(deps: Deps, address: &Addr, denom: String) -> StdResult<AccountResponse> {

    // Query the blockchain for the account balance
    let balance = deps.querier.query_balance(address, denom)?;

    Ok(AccountResponse {
        address: address.clone(),
        balance: balance.amount.u128(), 
    })
}
