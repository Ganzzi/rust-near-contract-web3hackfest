use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ AccountId, Timestamp, env, serde::{Serialize, Deserialize}};

use crate::category::{CategoryId, CategoryJson};
use crate::member::MemberJson;
use crate::submission::{SubmissionId, SubmissionJson};

pub type HackathonId = u64;

// Define the hackathon structure
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Hackathon {
    pub owner: AccountId,
    pub id: HackathonId,
    pub name: String,
    pub description: String,
    pub time: Timestamp,
    pub tags: Vec<String>,
    pub participants_list: Vec<AccountId>,
    pub submissions_list: Vec<SubmissionId>,
    pub categories_list: Vec<CategoryId>
}

// Implement the hackathon structure
impl Hackathon {
    pub fn new(
        id: HackathonId,
        payload: HackathonPayload
    ) -> Self {
        Hackathon {
            id,
            owner: env::signer_account_id(),
            name: payload.name,
            description: payload.description,
            time: payload.time,
            tags: payload.tags,
            participants_list: Vec::new(),
            submissions_list: Vec::new(),
            categories_list: Vec::new()
        }
    }
}

// Define the hackathon json structure (response)
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct HackathonJson {
    pub participants: Vec<MemberJson>,
    pub submissions: Vec<SubmissionJson>,
    pub categories: Vec<CategoryJson>
}

// Define the hackathon payload structure (request)
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct HackathonPayload {
    name: String,
    description: String,
    time: Timestamp,
    tags: Vec<String>,
}
