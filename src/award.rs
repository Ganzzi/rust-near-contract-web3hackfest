use near_sdk::{Balance, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};

pub type AwardId = u64;

// Define the Award structure
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Award {
    pub id: AwardId,
    pub name: String,
    pub price: Balance,
    pub winner: Option<AccountId>,
    pub is_awarded: bool
}

// Implement the Award structure
impl Award {
    pub fn new(id: AwardId,name: String, price: Balance) -> Self {
        Award { id: id, name, price, winner: None, is_awarded: false }
    }
}