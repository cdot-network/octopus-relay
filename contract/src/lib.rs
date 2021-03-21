// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json};
use std::collections::HashMap;

use near_sdk::{
    wee_alloc, env, near_bindgen, AccountId, 
    Balance, PromiseResult, BlockHeight,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub const TOKEN_ACCOUNT_ID: &str = "dev-1615435740118-2637667";

const NO_DEPOSIT: Balance = 0;
const SINGLE_CALL_GAS: u64 = 50_000_000_000_000;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Vote {
    Yes,
    No,
}

/// Describes the status of appchains
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum AppchainStatus {
    InQueue,
    OnVote,
    Frozen,
    Broken,
    Active,
}

impl Default for AppchainStatus {
    fn default() -> Self {
        AppchainStatus::Frozen
    }
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Delegation {
    account_id: String,
    amount: u64,
    block_height: BlockHeight,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Validator {
    account_id: String,
    id: String,
    ocw_id: String,
    weight: u64,
    staked_amount: u64,
    block_height: BlockHeight,
    delegations: Vec<Delegation>
}

impl Default for Validator {
    fn default() -> Self {
        Self {
            account_id: String::from(""),
            id: String::from(""),
            ocw_id: String::from(""),
            weight: 0,
            staked_amount: 0,
            block_height: 0,
            delegations: vec![]
        }
    }
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ValidatorSet {
    appchain_id: u32,
    sequence_number: u32,
    validators: Vec<Validator>,
}

#[derive(Clone, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Appchain {
    id: u32,
    founder_id: AccountId,
    appchain_name: String,
    runtime_url: String,
    runtime_hash: String,
    bond_tokens: u64,
    validator_set: HashMap<u32, ValidatorSet>,
    validators: Vec<Validator>,
    status: AppchainStatus,
    block_height: BlockHeight,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OctopusRelay {
    owner: AccountId,
    appchains: HashMap<u32, Appchain>,
    appchain_minium_validators: u32,
    minium_staking_amount: u64,
    total_staked_balance: u64,
}

impl Default for OctopusRelay {
    fn default() -> Self {
        env::panic(b"The contract should be initialized before usage")
    }
}

#[near_bindgen]
impl OctopusRelay {

    #[init]
    pub fn new(owner: AccountId, appchain_minium_validators: u32, minium_staking_amount: u64) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        Self {
            owner,
            appchains: HashMap::default(),
            total_staked_balance: 0,
            appchain_minium_validators,
            minium_staking_amount,
        }
    }
    
    #[payable]
    pub fn register_appchain(
        &mut self, 
        appchain_name: String, 
        runtime_url: String,
        runtime_hash: String,
        bond_tokens: u64,
    ) {
        let account_id = env::signer_account_id();

        let deposit = env::attached_deposit();

        // Cross-contract call to transfer OCT token
        let promise_transfer = env::promise_create(
            TOKEN_ACCOUNT_ID.to_string(),
            b"transfer_from", 
            json!({ 
                "owner_id": account_id,
                "new_owner_id": env::current_account_id(), 
                "amount": bond_tokens.to_string()
            }).to_string().as_bytes(),
            deposit,
            SINGLE_CALL_GAS,
        );

        // Check transfer token result and register appchain
        let promise_register = env::promise_then(
            promise_transfer,
            env::current_account_id(),
            b"check_transfer_and_register",
            json!({
                "account_id": account_id,
                "appchain_name": appchain_name,
                "runtime_url": runtime_url,
                "runtime_hash": runtime_hash,
                "bond_tokens": bond_tokens,
            }).to_string().as_bytes(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        );

        env::promise_return(promise_register);
    }

    pub fn check_transfer_and_register(
        &mut self,
        account_id: AccountId,
        appchain_name: String, 
        runtime_url: String,
        runtime_hash: String,
        bond_tokens: u64,
    ) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                env::log(b"Transfer token successful, start to register");

                let appchain_id = self.appchains.len() as u32;

                // Default validator set
                let mut validator_hash_map = HashMap::new();
                validator_hash_map.insert(0, ValidatorSet {
                    appchain_id,
                    sequence_number: 0,
                    validators: vec![],
                });

                let appchain = Appchain {
                    id: appchain_id,
                    founder_id: account_id.clone(),
                    appchain_name: appchain_name.clone(),
                    runtime_url,
                    runtime_hash,
                    bond_tokens,
                    validator_set: validator_hash_map,
                    validators: Vec::default(),
                    status: AppchainStatus::default(),
                    block_height: env::block_index(),
                };

                self.appchains.insert(appchain_id, appchain);
            }
            _ => panic!("Transfer token failed"),
        };
    }

    pub fn get_appchains(&self, from_index: u32, limit: u32) -> Vec<&Appchain> {
        (from_index..std::cmp::min(from_index + limit, self.appchains.len() as u32))
            .map(|index| self.appchains.get(&index).unwrap())
            .collect()
    }
    
    pub fn get_num_appchains(&self) -> usize {
        self.appchains.len()
    }

    /// Returns the total staking balance.
    pub fn get_total_staked_balance(&self) -> u64 {
        self.total_staked_balance
    }

    pub fn get_minium_staking_amount(&self) -> u64 {
        self.minium_staking_amount
    }

    pub fn get_appchain(&self, appchain_id: u32) -> Option<Appchain> {
        self.appchains.get(&appchain_id).cloned()
    }

    pub fn get_validator_set(&self, appchain_id: u32, seq_num: u32) -> Option<ValidatorSet> {
        let appchain = self.appchains.get(&appchain_id).expect("Appchain not found");

        appchain.validator_set.get(&seq_num).cloned()
    }

    // Returns the appchain current validator_set index
    pub fn get_curr_validator_set_index(&self, appchain_id: u32) -> u32 {
        let appchain = self.appchains.get(&appchain_id).expect("Appchain not found");
        appchain.validator_set.len() as u32 - 1
    }

    #[payable]
    pub fn staking(
        &mut self, 
        appchain_id: u32, 
        id: String,
        ocw_id: String,
        amount: u64,
    ) {
        
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();

        // Check amount
        assert!(amount >= self.minium_staking_amount, "Insufficient staking amount");

        if !self.appchains.contains_key(&appchain_id) {
            panic!("Appchain not found");
        }
     
         // Cross-contract call to transfer OCT token
        let promise_transfer = env::promise_create(
            TOKEN_ACCOUNT_ID.to_string(),
            b"transfer_from", 
            json!({ 
                "owner_id": account_id, 
                "new_owner_id": env::current_account_id(), 
                "amount": amount.to_string(),
            }).to_string().as_bytes(),
            deposit,
            SINGLE_CALL_GAS,
        );

        // Check transfer token result and staking
        let promise_staking = env::promise_then(
            promise_transfer,
            env::current_account_id(),
            b"check_transfer_and_staking",
            json!({
                "account_id": account_id,
                "appchain_id": appchain_id,
                "id": id,
                "ocw_id": ocw_id,
                "amount": amount,
            }).to_string().as_bytes(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        );

        env::promise_return(promise_staking);
    }

    pub fn check_transfer_and_staking(
        &mut self,
        account_id: AccountId,
        appchain_id: u32, 
        id: String,
        ocw_id: String,
        amount: u64,
    ) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                env::log(b"Transfer token successful, start to staking");

                let mut appchain = self.appchains.get(&appchain_id).cloned().expect("Appchain not found");
        
                for v in appchain.validators.iter() {
                    assert!(v.account_id != account_id, "You are already staked on the appchain!");
                }

                appchain.validators.push(Validator {
                    account_id: account_id.clone(),
                    id,
                    ocw_id,
                    weight: amount,
                    block_height: env::block_index(),
                    staked_amount: amount,
                    delegations: Vec::default(),
                });

                // Update state
                self.appchains.insert(appchain_id, appchain);
                self.total_staked_balance += amount;

                // Check to update validator set
                self.update_validator_set(appchain_id);

            },
            _ => panic!("Transfer token failed"),
        }
    }

    #[payable]
    pub fn staking_more(
        &mut self, 
        appchain_id: u32, 
        amount: u64,
    ) {
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();

        // Check amount
        assert!(amount >= self.minium_staking_amount, "Insufficient staking amount");

        let appchain = self.appchains.get(&appchain_id).cloned().expect("Appchain not found");
        appchain.validators.iter().find(|v| v.account_id == account_id).expect("You are not staked on the appchain");

        // Cross-contract call to transfer OCT token
        let promise_transfer = env::promise_create(
            TOKEN_ACCOUNT_ID.to_string(),
            b"transfer_from", 
            json!({ 
                "owner_id": account_id, 
                "new_owner_id": env::current_account_id(), 
                "amount": amount.to_string(),
            }).to_string().as_bytes(),
            deposit,
            SINGLE_CALL_GAS,
        );

        // Check transfer token result and staking_more
        let promise_staking_more = env::promise_then(
            promise_transfer,
            env::current_account_id(),
            b"check_transfer_and_staking_more",
            json!({
                "account_id": account_id,
                "appchain_id": appchain_id,
                "amount": amount,
            }).to_string().as_bytes(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        );

        env::promise_return(promise_staking_more);
    }

    pub fn check_transfer_and_staking_more(
        &mut self,
        account_id: AccountId,
        appchain_id: u32, 
        amount: u64,
    ) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                env::log(b"Transfer token successful, start to staking_more");

                let mut appchain = self.appchains.get(&appchain_id).cloned().expect("Appchain not found");
                
                let mut found = false;
                for v in appchain.validators.iter_mut() {
                    if v.account_id == account_id {
                        v.staked_amount += amount;
                        v.weight += amount;
                        found = true;
                    }
                }

                if !found {
                    panic!("You are not staked on the appchain");
                }

                // Update state
                self.appchains.insert(appchain_id, appchain);
                self.total_staked_balance += amount;

                // Check to update validator set
                self.update_validator_set(appchain_id);
            },
            _ => panic!("Transfer token failed"),
        }
    }

    pub fn unstaking(&mut self, appchain_id: u32) {
        
        let account_id = env::signer_account_id();
        
        env::log(format!("Account '{}' unstaking", account_id).as_bytes());

        let appchain = self.appchains.get(&appchain_id).cloned().expect("Appchain not found");
     
        let validator = appchain.validators.iter().find(|v| v.account_id == account_id).expect("You are not staked on the appchain");

         // Cross-contract call to transfer OCT token
        let promise_transfer = env::promise_create(
            TOKEN_ACCOUNT_ID.to_string(),
            b"transfer_from", 
            json!({ 
                "owner_id": env::current_account_id(), 
                "new_owner_id": account_id, 
                "amount": validator.staked_amount.to_string(),
            }).to_string().as_bytes(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        );

        // Check transfer token result and staking
        let promise_staking = env::promise_then(
            promise_transfer,
            env::current_account_id(),
            b"check_transfer_and_unstaking",
            json!({
                "appchain_id": appchain_id,
                "account_id": account_id,
                "amount": validator.staked_amount,
            }).to_string().as_bytes(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        );

        env::promise_return(promise_staking);
    }

    pub fn check_transfer_and_unstaking(&mut self, appchain_id: u32, account_id: AccountId, amount: u64) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                env::log(format!("Transfer token successful, start to unstaking, account_id: {}", account_id).as_bytes());
                let mut appchain = self.appchains.get(&appchain_id).cloned().expect("Appchain not found");

                // Remove the validator
                appchain.validators.retain(|v| v.account_id != account_id);

                // Update state
                self.appchains.insert(appchain_id, appchain);
                self.total_staked_balance -= amount;

                // Check to update validator set
                self.update_validator_set(appchain_id);
                
            },
            _ => panic!("Transfer token failed"),
        };
        
    }

    pub fn active_appchain(&mut self, appchain_id: u32) {
        let mut appchain = self.appchains.get(&appchain_id).cloned().expect("Appchain not found");
        let account_id = env::signer_account_id();

        // Only the appchain founder can do this
        assert!(account_id == appchain.founder_id, "You're not the appchain founder");

        // Can only active a frozen appchain
        assert!(appchain.status == AppchainStatus::Frozen, "Appchain status incorrect");
        
        // Check validators
        assert!(
            appchain.validators.len() as u32 >= self.appchain_minium_validators, 
            "Insufficient number of appchain validators"
        );

        appchain.status = AppchainStatus::Active;

        // Update state
        self.appchains.insert(appchain_id, appchain);

        // Check to update validator set
        self.update_validator_set(appchain_id);
    }

    /*
        Update validator set, is called after the appchain validators or status updated
    */
    fn update_validator_set(&mut self, appchain_id: u32) -> bool {
        let mut appchain = self.appchains.get(&appchain_id).cloned().unwrap();

        let appchain_curr_validator_set_idx = self.get_curr_validator_set_index(appchain_id);
        let mut validator_set = appchain.validator_set.get(&appchain_curr_validator_set_idx).unwrap().clone();

        // Check status
        if appchain.status != AppchainStatus::Active {
            return false;
        }

        let mut changed = false;
        
        let validators_len = appchain.validators.len() as u32;
        if validators_len < self.appchain_minium_validators {
            appchain.status = AppchainStatus::Frozen;
            validator_set.validators = vec![];
            changed = true;
        } else {
            appchain.validators.sort_by(|a, b| b.weight.cmp(&a.weight));
        }

        // Compare sorted array
        if !changed {
            let max_index = appchain.validators.len().max(validator_set.validators.len());
            let default_validator = Validator::default();
            for i in 0..max_index {
                let v = validator_set.validators.get(i).unwrap_or(&default_validator);
                let av = appchain.validators.get(i).unwrap_or(&default_validator);
                if av.account_id != v.account_id {
                    changed = true;
                    validator_set.validators = appchain.validators.clone();
                    break;
                }
            }
        }

        // Update state
        if changed {
            validator_set.sequence_number += 1;

            appchain.validator_set.insert(appchain_curr_validator_set_idx + 1, validator_set);
            self.appchains.insert(appchain_id, appchain);
        }

        true

    }

}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    
}
