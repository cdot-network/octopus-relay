// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json};
use std::collections::HashMap;

use near_sdk::{
    wee_alloc, env, near_bindgen, AccountId, 
    Balance, PromiseResult, EpochHeight,
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
        AppchainStatus::Active
    }
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Delegation {
    account_id: String,
    amount: u64,
    epoch_height: EpochHeight,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Validator {
    account_id: String,
    id: String,
    ocw_id: String,
    weight: u64,
    staked_amount: u64,
    delegations: Vec<Delegation>
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeRecord {
    validator: Validator,
    epoch_height: EpochHeight,
    weight: u64,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ValidatorSet {
    appchain_id: u32,
    validator_set_index: u32,
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
    num_validators: u64,
    bond_tokens: u64,
    validator_set: HashMap<u32, ValidatorSet>,
    stake_records: Vec<StakeRecord>,
    status: AppchainStatus,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OctopusRelay {
    council: Vec<AccountId>,
    appchains: HashMap<u32, Appchain>,
    minium_stake: u64,
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
    pub fn new(council: Vec<AccountId>, minium_stake: u64) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        let mut octopus_relay = Self {
            council: Vec::default(),
            appchains: HashMap::default(),
            total_staked_balance: 0,
            minium_stake: minium_stake,
        };

        for account_id in council {
            octopus_relay.council.push(account_id);
        }

        octopus_relay
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
                    validator_set_index: 0,
                    validators: vec![],
                });

                let appchain = Appchain {
                    id: appchain_id,
                    founder_id: account_id.clone(),
                    appchain_name: appchain_name.clone(),
                    runtime_url,
                    runtime_hash,
                    num_validators: 0,
                    bond_tokens,
                    validator_set: validator_hash_map,
                    stake_records: Vec::default(),
                    status: AppchainStatus::default(),
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

    pub fn get_appchain(&self, appchain_id: u32) -> Option<Appchain> {
        self.appchains.get(&appchain_id).cloned()
    }

    pub fn get_validator_set(&self, appchain_id: u32, index: u32) -> Option<ValidatorSet> {
        let appchain = self.appchains.get(&appchain_id).expect("Appchain not found");

        appchain.validator_set.get(&index).cloned()
    }

    // Returns the appchain current validator_set index
    pub fn get_curr_validator_set_index(&self, appchain_id: u32) -> u32 {
        let appchain = self.appchains.get(&appchain_id).expect("Appchain not found");
        appchain.validator_set.len() as u32 - 1
    }

    #[payable]
    pub fn stake(
        &mut self, 
        appchain_id: u32, 
        id: String,
        ocw_id: String,
        amount: u64,
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
                "amount": amount.to_string(),
            }).to_string().as_bytes(),
            deposit,
            SINGLE_CALL_GAS,
        );

        // Check transfer token result and stake
        let promise_stake = env::promise_then(
            promise_transfer,
            env::current_account_id(),
            b"check_transfer_and_stake",
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

        env::promise_return(promise_stake);
    }

    pub fn check_transfer_and_stake(
        &mut self,
        account_id: AccountId,
        appchain_id: u32, 
        id: String,
        ocw_id: String,
        amount: u64,
    ) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                env::log(b"Transfer token successful, start to stake");

                let mut appchain = self.appchains.get(&appchain_id).cloned().expect("Appchain not found");
        
                let appchain_curr_validator_set_idx = self.get_curr_validator_set_index(appchain_id);
                let mut validator_set = appchain.validator_set.get(&appchain_curr_validator_set_idx).unwrap().clone();
                
                for r in appchain.stake_records.iter() {
                    assert!(r.validator.account_id != account_id, "You are already staked on the appchain!");
                }

                let validator = Validator {
                    account_id: account_id.clone(),
                    id,
                    ocw_id,
                    weight: amount,
                    staked_amount: amount,
                    delegations: Vec::default(),
                };

                validator_set.validators.push(validator.clone());

                // Record
                appchain.stake_records.push(StakeRecord {
                    validator,
                    epoch_height: env::epoch_height(),
                    weight: amount,
                });

                // Update state
                self.appchains.insert(appchain_id, appchain);

                // Update validator set
                self.update_validator_set(appchain_id);

            },
            _ => panic!("Transfer token failed"),
        }
    }

    pub fn unstake(&mut self, appchain_id: u32) {
        
        let account_id = env::signer_account_id();
        
        env::log(format!("Account '{}' unstake", account_id).as_bytes());

        let appchain = self.appchains.get(&appchain_id).cloned().expect("Appchain not found");
     
        let record = appchain.stake_records.iter().find(|r| r.validator.account_id == account_id).expect("You are not staked on the appchain");

         // Cross-contract call to transfer OCT token
        let promise_transfer = env::promise_create(
            TOKEN_ACCOUNT_ID.to_string(),
            b"transfer_from", 
            json!({ 
                "owner_id": env::current_account_id(), 
                "new_owner_id": account_id, 
                "amount": record.validator.staked_amount.to_string(),
            }).to_string().as_bytes(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        );

        // Check transfer token result and stake
        let promise_stake = env::promise_then(
            promise_transfer,
            env::current_account_id(),
            b"check_transfer_and_unstake",
            json!({
                "appchain_id": appchain_id,
                "account_id": account_id,
            }).to_string().as_bytes(),
            NO_DEPOSIT,
            SINGLE_CALL_GAS,
        );

        env::promise_return(promise_stake);
    }

    pub fn check_transfer_and_unstake(&mut self, appchain_id: u32, account_id: AccountId) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                env::log(format!("Transfer token successful, start to unstake, account_id: {}", account_id).as_bytes());
                let mut appchain = self.appchains.get(&appchain_id).cloned().expect("Appchain not found");

                // Remove the stake record
                appchain.stake_records.retain(|record| record.validator.account_id != account_id);

                self.appchains.insert(appchain_id, appchain);

                // Update validator set
                self.update_validator_set(appchain_id);
            },
            _ => panic!("Transfer token failed"),
        };
        
    }

    fn update_validator_set(&mut self, appchain_id: u32) {
        let mut appchain = self.appchains.get(&appchain_id).cloned().expect("Appchain not found");

        // Get current validator set
        let appchain_curr_validator_set_idx = self.get_curr_validator_set_index(appchain_id);
        let mut validator_set = appchain.validator_set.get(&appchain_curr_validator_set_idx).unwrap().clone();

        let mut records = appchain.stake_records.clone();
        let mut changed = false;

        if records.len() < 2 {
            // Clear the vector
            if validator_set.validators.len() > 0 {
                validator_set.validators = vec![];
                changed = true;
            } else {
                return;
            }
        } else {
            records.sort_by(|a, b| b.epoch_height.cmp(&a.epoch_height));
            records.sort_by(|a, b| b.weight.cmp(&a.weight));
        }
        
        if !changed {
            if validator_set.validators.len() < 2 {
                for i in 0..2 {
                    let tmp_record = records.get(i).unwrap();
                    validator_set.validators.insert(i, tmp_record.validator.clone());
                }
                changed = true;
            } else {
                for (i, v) in validator_set.validators.iter_mut().enumerate() {
                    let tmp_record = records.get(i).unwrap();
                    if v.account_id != tmp_record.validator.account_id {
                        *v = tmp_record.validator.clone();
                        changed = true;
                    }
                }
            }
        }

        if changed {
            validator_set.validator_set_index += 1;
            appchain.validator_set.insert(appchain_curr_validator_set_idx + 1, validator_set);
            self.appchains.insert(appchain_id, appchain);
        }

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
