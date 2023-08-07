use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ AccountId, serde::{Serialize, Deserialize}};

pub type SubmissionId = u64;

use crate::category::{CategoryId, CategoryJson};
use crate::member::MemberJson;

// Define the Submission structure
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Submission {
    pub id: SubmissionId,
    pub categories: Vec<CategoryId>,
    pub members: Vec<AccountId>
}

// Implement the Submission structure
impl Submission {
    pub fn new(submission_id: u64, categories: Vec<u64>, members: Vec<AccountId>) -> Self {
        Submission { id: submission_id, categories, members }
    }
} 

// Define the Detail Submission Json structure
#[derive(BorshDeserialize, BorshSerialize,Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SubmissionJson {
    pub id: SubmissionId,
    pub categories: Vec<CategoryJson>,
    pub members: Vec<MemberJson>
}
