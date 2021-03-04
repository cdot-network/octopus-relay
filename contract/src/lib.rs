// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

use near_sdk::json_types::U128;

use near_sdk::collections::{UnorderedSet, UnorderedMap, Vector};
use near_sdk::wee_alloc;
use near_sdk::{
    env, near_bindgen, AccountId, Balance
};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Validator {
    pub appchain_id: u64,
    pub account_id: AccountId,
    pub appchain_account: String,
    pub staked_balance: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Appchain {
    pub founder_id: AccountId,
    pub appchain_name: String,
    pub runtime_url: String,
    pub runtime_hash: String,
    pub num_validators: u64,
    pub staked_balance: Balance,
    pub status: AppchainStatus,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct RewardFeeFraction {
    pub numerator: u32,
    pub denominator: u32,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CdotRelay {
    pub council: UnorderedSet<AccountId>,
    pub appchains: Vector<Appchain>,
    pub validators: UnorderedMap<AccountId, Validator>,
    pub total_staked_balance: Balance,
}

impl Default for CdotRelay {
    fn default() -> Self {
        env::panic(b"The contract should be initialized before usage")
    }
}

#[near_bindgen]
impl CdotRelay {

    #[init]
    pub fn new(council: Vec<AccountId>) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        let mut cdot_relay = Self {
            council: UnorderedSet::new(b"c".to_vec()),
            appchains: Vector::new(b"a".to_vec()),
            validators: UnorderedMap::new(b"v".to_vec()),
            total_staked_balance: 0,
        };

        for account_id in council {
            cdot_relay.council.insert(&account_id);
        }

        cdot_relay
    }
    
    #[payable]
    pub fn register_appchain(
        &mut self, 
        appchain_name: String, 
        runtime_url: String,
        runtime_hash: Strng,
    ) -> u64 {
        let account_id = env::signer_account_id();
        let amount = env::attached_deposit();

        env::log(format!(
            "Account '{}' register appchain, rumtime_url: '{}', staked: '{}'", 
            &account_id, runtime_url, amount
        ).as_bytes());
        
        let appchain = Appchain {
            founder_id: account_id.clone(),
            appchain_name,
            runtime_url,
            runtime_hash,
            num_validators: 0,
            staked_balance: amount,
            status: AppchainStatus::default(),
        };

        self.total_staked_balance += amount;
        self.appchains.push(&appchain);
        self.appchains.len() - 1
    }

    pub fn get_appchain(&self, id: u64) -> Appchain {
        self.appchains.get(id).expect("Appchain not found")
    }
    
    pub fn get_num_appchains(&self) -> u64 {
        self.appchains.len()
    }

    /// Returns the total staking balance.
    pub fn get_total_staked_balance(&self) -> U128 {
        self.total_staked_balance.into()
    }

    pub fn get_num_validators(&self) -> u64 {
        self.validators.len()
    }
 
    pub fn get_appchains(&self, from_index: u64, limit: u64) -> Vec<Appchain> {
        (from_index..std::cmp::min(from_index + limit, self.appchains.len()))
            .map(|index| self.appchains.get(index).unwrap())
            .collect()
    }

    pub fn get_appchain_validators(&self, id: u64) -> Vec<Validator> {
        self.validators.iter()
            .filter(|(_, item)| item.appchain_id == id)
            .map(|(_, item)| item)
            .collect()
    }

    #[payable]
    pub fn stake_to_be_validator(&mut self, appchain_id: u64, appchain_account: String) {
        let account_id = env::predecessor_account_id();
        let amount = env::attached_deposit();

        env::log(format!("Account '{}' stake to be validator, appchain_id: '{}'", &account_id, &appchain_id).as_bytes());

        assert!(self.validators.get(&account_id).is_none(), "Already a validator");
        let mut appchain = self.appchains.get(&appchain_id).expect("Appchain not found");

        let validator = Validator {
            appchain_id,
            account_id: account_id.clone(),
            appchain_account,
            staked_balance: amount
        };

        appchain.num_validators += 1;
        appchain.staked_balance += amount;

        self.total_staked_balance += amount;
        self.validators.insert(&account_id, &validator);
    }

    pub fn unstake(&mut self, appchain_id: u64) {
        let account_id = env::predecessor_account_id();
        assert!(self.validators.get(&account_id).is_some(), "You're not an appchain validator");
        let mut appchain = self.appchains.get(&appchain_id).expect("Appchain not found");

        let validator = self.validators.get(&account_id);

        appchain.num_validators -= 1;
        appchain.staked_balance -= validator.staked_balance;

        self.total_staked_balance -= amount;
        self.validators.remove(&account_id);
    }

    pub fn freeze_appchain(&mut self, appchain_id: u64) {
        let account_id = env::predecessor_account_id();
        env::log(format!("Account '{}' freeze appchain, id: '{}'", &account_id, &appchain_id).as_bytes());
        let mut appchain = self.appchains.get(appchain_id).expect("Appchain not found");

        assert!(appchain.founder_id == account_id, "Not appchain founder");

        appchain.status = AppchainStatus::Frozen;
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
