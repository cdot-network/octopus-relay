// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use std::collections::HashMap;

use near_sdk::{
    wee_alloc, env, near_bindgen, AccountId, 
    Balance, Gas, Promise,
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static TOKEN_ACCOUNT: &str = "dev-1615435740118-2637667";

const NO_DEPOSIT: Balance = 0;
const BASIC_GAS: Gas = 5_000_000_000_000;

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
pub struct Validator {
    ocw_id: String,
    id: String,
    staked_balance: u64,
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ValidatorSet {
    appchain_name: String,
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
    bond_balance: Balance,
    curr_validator_set_index: u32,
    validator_set: HashMap<u32, ValidatorSet>,
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
    total_staked_balance: Balance,
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
        bond_balance: Balance,
    ) {
        let account_id = env::signer_account_id();
        
        env::log(format!(
            "Account '{}' register appchain, rumtime_url: '{}', bond_balance: '{}'", 
            &account_id, runtime_url, bond_balance,
        ).as_bytes());
        
        // Transfer octopus token from the founder account to the relay
        // #TODO: some validations
        // Promise::new(TOKEN_ACCOUNT.to_string()).function_call(
        //     b"transfer_from".to_vec(), 
        //     format!(
        //         "{{\"owner_id\": \"{}\", \"new_owner_id\": \"{}\", \"amount\": \"{}\"}}", 
        //         account_id, 
        //         env::current_account_id(),
        //         bond_balance,
        //     ).into_bytes(),
        //     NO_DEPOSIT,
        //     BASIC_GAS,
        // );

        // Default validator set
        let mut validator_hash_map = HashMap::new();
        validator_hash_map.insert(0, ValidatorSet {
            appchain_name: appchain_name.clone(),
            validator_set_index: 0,
            validators: vec![],
        });

        let appchain_id = self.appchains.len() as u32;
        let appchain = Appchain {
            id: appchain_id,
            founder_id: account_id.clone(),
            appchain_name: appchain_name.clone(),
            runtime_url,
            runtime_hash,
            num_validators: 0,
            bond_balance,
            curr_validator_set_index: 0,
            validator_set: validator_hash_map,
            status: AppchainStatus::default(),
        };

        self.appchains.insert(appchain_id, appchain);

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
    pub fn get_total_staked_balance(&self) -> Balance {
        self.total_staked_balance

        // #TODO: Get balance from the token contract
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
        appchain.curr_validator_set_index
    }

    pub fn stake_to_be_validator(
        &mut self, 
        appchain_id: u32, 
        appchain_account: String,
        amount: u64,
    ) {
        let account_id = env::predecessor_account_id();
        env::log(format!(
            "Account '{}' stake to be validator, appchain_id: '{}', amount: '{}'", 
            &account_id, 
            &appchain_id,
            amount,
        ).as_bytes());

        // assert!(
        //     amount >= self.minium_stake, 
        //     format!("Stake amount must gretter than '{}'", self.minium_stake)
        // );

        let mut appchain = self.appchains.get(&appchain_id).cloned().expect("Appchain not found");
        let appchain_curr_validator_set_idx = appchain.curr_validator_set_index;

        let mut validator_set = appchain.validator_set.get(&appchain_curr_validator_set_idx).unwrap().clone();

        let validator = Validator {
            id: account_id.clone(),
            ocw_id: appchain_account,
            staked_balance: amount
        };

        validator_set.validators.push(validator);

        appchain.validator_set.insert(appchain_curr_validator_set_idx + 1, validator_set);
        appchain.curr_validator_set_index = appchain_curr_validator_set_idx + 1;

        // update state
        self.appchains.insert(appchain_id, appchain);
      
        // Transfer octopus token from the founder account to the relay
        // #TODO: some validations
        // Promise::new(TOKEN_ACCOUNT.to_string()).function_call(
        //     b"transfer_from".to_vec(), 
        //     format!(
        //         "{{\"owner_id\": \"{}\", \"new_owner_id\": \"{}\", \"amount\": \"{}\"}}", 
        //         account_id, 
        //         env::current_account_id(),
        //         amount,
        //     ).into_bytes(),
        //     NO_DEPOSIT,
        //     BASIC_GAS,
        // );
    }

    // pub fn get_num_validators(&self) -> u64 {
    //     self.validators.len().into()
    // }
 
    // pub fn get_appchain_validators(&self, id: u64) -> Vec<Validator> {
    //     self.validators.iter()
    //         .filter(|(_, item)| item.appchain_id == id)
    //         .map(|(_, item)| item)
    //         .collect()
    // }

    // #[payable]
    // pub fn stake_to_be_validator(&mut self, appchain_id: u64, appchain_account: String) {
    //     let account_id = env::predecessor_account_id();
    //     let amount = env::attached_deposit();

    //     env::log(format!("Account '{}' stake to be validator, appchain_id: '{}'", &account_id, &appchain_id).as_bytes());

    //     assert!(self.validators.get(&account_id).is_none(), "Already a validator");
    //     let mut appchain = self.appchains.get(&appchain_id).expect("Appchain not found");

    //     let validator = Validator {
    //         appchain_id,
    //         account_id: account_id.clone(),
    //         appchain_account,
    //         staked_balance: amount
    //     };

    //     appchain.num_validators += 1;
    //     appchain.staked_balance += amount;

    //     self.total_staked_balance += amount;
    //     self.validators.insert(&account_id, &validator);
    // }

    // pub fn unstake(&mut self, appchain_id: u64) {
    //     let account_id = env::predecessor_account_id();
    //     assert!(self.validators.get(&account_id).is_some(), "You're not an appchain validator");
    //     let mut appchain = self.appchains.get(&appchain_id).expect("Appchain not found");

    //     let validator = self.validators.get(&account_id);
    //     let amount = validator.staked_balance;

    //     appchain.num_validators -= 1;
    //     appchain.staked_balance -= amount;

    //     self.total_staked_balance -= amount;
    //     self.validators.remove(&account_id);
    // }

    // pub fn freeze_appchain(&mut self, appchain_id: u64) {
    //     let account_id = env::predecessor_account_id();
    //     env::log(format!("Account '{}' freeze appchain, id: '{}'", &account_id, &appchain_id).as_bytes());
    //     let mut appchain = self.appchains.get(appchain_id).expect("Appchain not found");

    //     assert!(appchain.founder_id == account_id, "Not appchain founder");

    //     appchain.status = AppchainStatus::Frozen;
    // }

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
