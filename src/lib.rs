use award::{AwardId, Award};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{near_bindgen, AccountId, Balance, env, Promise, PanicOnDefault};

// declare module
mod member;
mod hackathon;
mod category;
mod submission;
mod award;

// import module
use member::{Member, MemberJson, MemberJsonDetail};
use hackathon::{Hackathon, HackathonPayload, HackathonId, HackathonJson};
use submission::{Submission, SubmissionId, SubmissionJson};
use category::{Category, CategoryId, CategoryJson};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct HackathonContract {
    pub members_list: UnorderedSet<AccountId>,
    pub hackathons_list: UnorderedSet<HackathonId>,

    pub members: LookupMap<AccountId, Member>,
    pub awards: LookupMap<AwardId, Award>,
    pub hackathons: LookupMap<HackathonId, Hackathon>,
    pub submissions: LookupMap<SubmissionId, Submission>,
    pub categories: LookupMap<CategoryId, Category>,

    pub next_submission_id: SubmissionId,
    pub next_category_id: CategoryId,
    pub next_award_id: AwardId,
    pub next_hackathon_id: HackathonId,
}

// Implement the contract structure
#[near_bindgen]
impl HackathonContract {
    #[init]
    pub fn init() -> Self {
        Self {
            members_list: UnorderedSet::new(b"m".to_vec()),
            hackathons_list: UnorderedSet::new(b"m".to_vec()),
            members: LookupMap::new(b"members".to_vec()),
            hackathons: LookupMap::new(b"hackathons".to_vec()),
            submissions: LookupMap::new(b"submissions".to_vec()),
            categories: LookupMap::new(b"categories".to_vec()),
            awards: LookupMap::new(b"awards".to_vec()),
            next_hackathon_id: 0,
            next_submission_id: 0,
            next_category_id: 0, 
            next_award_id: 0,
        }
    }

    pub fn add_member(
        &mut self, id: AccountId, name: String, image: Option<String>, bio: Option<String>
    ) {
        let member = Member::new(&id, name, image, bio);

        self.members_list.insert(&id);
        self.members.insert(&id, &member);
    }

    pub fn add_hackathon(
        &mut self,
        payload: HackathonPayload
    ) -> HackathonId {
        let hackathon_id = self.next_hackathon_id;
        self.next_hackathon_id += 1;

        let hackathon = Hackathon::new(
            hackathon_id,
            payload
        );

        self.hackathons_list.insert(&hackathon_id);
        self.hackathons.insert(&hackathon_id, &hackathon);

        hackathon_id
    }

    pub fn add_category(
        &mut self, hackathon_id: HackathonId, name: String
    ) -> Option<u64> {
        if let Some(mut hackathon) = self.hackathons.get(&hackathon_id) {
            let account_id = env::signer_account_id();

            assert_eq!(account_id, hackathon.owner, "Not owner");

            let category_id = self.next_category_id;
            self.next_category_id += 1;

            let category = Category::new(category_id, name);

            self.categories.insert(&category_id, &category);

            hackathon.categories_list.push(category_id);
            self.hackathons.insert(&hackathon_id, &hackathon);

            Some(category_id)
        } else {
            None
        }
    }

    pub fn add_award(
        &mut self,
        hackathon_id: HackathonId,
        category_id: CategoryId,
        name: String,
        total: Balance,
    ) -> Option<AwardId> {
        assert_eq!(
            env::signer_account_id(), 
            self.hackathons.get(&hackathon_id).unwrap().owner, 
            "Not owner"
        );
        
        if let Some(mut category) = self.categories.get(&category_id) {
            let award_id = self.next_award_id;
            self.next_award_id += 1;
            
            let award = Award::new(award_id, name, total);

            self.awards.insert(&award_id, &award);

            category.awards.push(award_id);
            self.categories.insert(&category_id, &category);
            

            Some(award_id)
        } else {
            return None;
        }  
    }
      
    pub fn join_hackathon(&mut self, hackathon_id: u64) {
        let account_id = env::signer_account_id();
        if let Some(mut member) = self.members.get(&account_id) {
            assert_eq!(account_id, self.members.get(&account_id).unwrap().id, "Not a member");
    
            if let Some(mut hackathon) = self.hackathons.get(&hackathon_id) {
                assert_ne!(account_id, hackathon.owner, "Can't join your own");
                assert_eq!(hackathon.participants_list.contains(&account_id), false, "Existing in hackathon");
            
                hackathon.participants_list.push(account_id.clone());
                self.hackathons.insert(&hackathon_id, &hackathon);
            }

            member.joined_hackathons.push(hackathon_id);
            self.members.insert(&account_id, &member);
        }
    }

    pub fn submit_project(&mut self, hackathon_id: HackathonId, categories: Vec<CategoryId>, members: Vec<AccountId>) {
        let account_id = env::signer_account_id();
        
        let submission_id: SubmissionId = self.next_submission_id;
        let submission = Submission::new(submission_id, categories, members);

        self.submissions.insert(&submission_id, &submission);

        if let Some(mut hackathon) = self.hackathons.get(&hackathon_id) {
            assert_eq!(hackathon.participants_list.contains(&account_id), true, "Not participated");
            
            hackathon.submissions_list.push(submission_id);
            self.hackathons.insert(&hackathon_id, &hackathon);
        }

        self.next_submission_id += 1;
    }
    
    pub fn judge_winner(
        &mut self, hackathon_id: HackathonId, 
        category_id: CategoryId, 
        award_id: AwardId,
        winner: AccountId
    ) {
        assert_eq!(winner, self.members.get(&winner).unwrap().id, "Winner is not a member");
        let account_id = env::signer_account_id();

        if let Some(hackathon) = self.hackathons.get(&hackathon_id) {
            assert_eq!(account_id, hackathon.owner, "Not nowner");
            assert_eq!(hackathon.participants_list.contains(&winner), true, "Not participated");
            assert_eq!(hackathon.categories_list.contains(&category_id), true, "category is not in hackathon");


            if let Some(category) = self.categories.get(&category_id) {
                assert_eq!(category.awards.contains(&award_id), true, "award is not in category");      
            }

            if let Some(mut award) = self.awards.get(&award_id) {
                assert_eq!(award.winner, None, "Winner judged");
                
                award.winner = Some(winner);

                self.awards.insert(&award_id, &award);
            }
        }
    }

    #[payable]
    pub fn award_winner(
        &mut self, hackathon_id: HackathonId, category_id: CategoryId, award_id: AwardId
    ) {
        let account_id = env::signer_account_id();

        if let Some(hackathon) = self.hackathons.get(&hackathon_id) {
            assert_eq!(account_id, hackathon.owner, "Not nowner");
            assert_eq!(hackathon.categories_list.contains(&category_id), true, "category is not in hackathon");

        }

        if let Some(category) = self.categories.get(&category_id) {
            assert_eq!(category.awards.contains(&award_id), true, "award is not in category");
        }  

        if let Some(mut award) = self.awards.get(&award_id) {
            assert_eq!(award.is_awarded, false, "award awarded");
            assert_eq! (env::attached_deposit(), award.price, "attached deposit should be equal to the award");

            let receiver = award.winner.clone().unwrap();

            Promise::new(receiver).transfer(award.price);
            award.is_awarded = true;

            self.awards.insert(&award_id, &award);
        }
       
    }
  
    // PUBLIC GETTER FUNCTION
    pub fn get_all_hackathons(&self) -> Vec<Hackathon> {
        let mut hackathons = Vec::new();

        for hackathon in self.hackathons_list.iter() {
            let rs = self.get_hackathon_by_id(hackathon).unwrap();
            hackathons.push(rs);
        }

        hackathons
    }

    pub fn get_detail_hackathon_by_id(&self, hackathon_id: HackathonId) -> Option<HackathonJson> {
        let hackathon = self.hackathons.get(&hackathon_id).unwrap();
        let mut pars = Vec::new();
        let mut cats = Vec::new();
        let mut subs = Vec::new();

        for p_id in hackathon.participants_list.iter() {
            let rs = self.get_member_by_id(p_id.clone()).unwrap();
            pars.push(rs);
        }
        
        for p_id in hackathon.categories_list.iter() {
            let rs = self.get_category_by_id(*p_id).unwrap();
            cats.push(rs);
        }

        for p_id in hackathon.submissions_list.iter() {
            let rs = self.get_submission_by_id(*p_id).unwrap();
            subs.push(rs);
        }


        Some(HackathonJson{
            participants: pars,
            submissions: subs,
            categories: cats
        })
    }

    pub fn get_user_information_by_id(&self, id: AccountId) -> Option<MemberJsonDetail> {
        match self.members.get(&id) {
            Some(user) => {
                let mut c_hack = Vec::new();
                let mut p_hack = Vec::new();

                for hackathon in user.created_hackathons {
                    c_hack.push(self.get_hackathon_by_id(hackathon).unwrap())
                }

                for hackathon in user.joined_hackathons {
                    p_hack.push(self.get_hackathon_by_id(hackathon).unwrap())
                }

                Some(MemberJsonDetail {
                    id: user.id,
                    name: user.name,
                    image: user.image,
                    bio: user.bio,
                    joined_hackathons: p_hack,
                    created_hackathons: c_hack
                })
            },
            None => None,
        }
    }

    // PRIVATE GETTER FUNCTION
    fn get_hackathon_by_id(&self, hackathon_id: HackathonId) -> Option<Hackathon> {
        if let Some(result) = self.hackathons.get(&hackathon_id) {
            Some(result)
        } else {
            None
        }
    }

    fn get_category_by_id(&self, category_id: CategoryId) -> Option<CategoryJson> {
        if let Some(result) = self.categories.get(&category_id) {
            // let mut prizes: Vec<String> = Vec::new();
            let mut awards = Vec::new();

            for i in result.awards.iter() {
                // let prize = self.get_prize_by_id(*i).unwrap();
                // prizes.push(prize);
                let a = self.get_award_by_id(*i).unwrap();
                awards.push(a);
                
            }

            let category_json: CategoryJson = CategoryJson {
                name: result.name,
                id: result.id,
                awards: awards
                // prizes_list: prizes
            };
            Some(category_json)
        } else {
            None
        }
    }

    fn get_award_by_id(&self, award_id: AwardId) -> Option<Award> {
        if let Some(rs) = self.awards.get(&award_id) {
            Some(rs)
        } else {
            None
        }
    }
    
    fn get_member_by_id(&self, member_id: AccountId) -> Option<MemberJson> {
        if let Some(result) = self.members.get(&member_id) {

            Some(
                MemberJson {
                    name: result.name,
                    id: result.id,
                    image: result.image,
                    bio: result.bio,
                }
            )
        } else {
            None
        }
    }

    fn get_submission_by_id(&self, submission_id: SubmissionId) -> Option<SubmissionJson> {
        if let Some(result) = self.submissions.get(&submission_id) {
            let mut pars = Vec::new();
            let mut cats = Vec::new();

            for p_id in result.categories.iter() {
                let cat = self.get_category_by_id(*p_id).unwrap();
                cats.push(cat);
            }

            for p_id in result.members.iter() {
                let par = self.get_member_by_id(p_id.clone()).unwrap();
                pars.push(par);
            }

            Some(
                SubmissionJson { id: result.id, categories: cats, members: pars }
            )
        } else {
            None
        }
    }
}
