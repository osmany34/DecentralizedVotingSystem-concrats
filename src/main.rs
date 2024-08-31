use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;

#[derive(Clone, CandidType, Deserialize)]
struct Proposal {
    id: u64,
    description: String,
    yes_votes: u64,
    no_votes: u64,
    active: bool,
    creator: String,
}

#[derive(Default)]
struct VotingContract {
    proposals: HashMap<u64, Proposal>,
    next_proposal_id: u64,
}

impl VotingContract {
    fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            next_proposal_id: 1,
        }
    }

    fn create_proposal(&mut self, description: String, creator: String) -> u64 {
        let proposal_id = self.next_proposal_id;
        self.next_proposal_id += 1;

        let proposal = Proposal {
            id: proposal_id,
            description,
            yes_votes: 0,
            no_votes: 0,
            active: true,
            creator,
        };

        self.proposals.insert(proposal_id, proposal);
        proposal_id
    }

    fn vote(&mut self, proposal_id: u64, vote: bool) -> Result<(), String> {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if !proposal.active {
                return Err("Proposal is no longer active.".to_string());
            }

            if vote {
                proposal.yes_votes += 1;
            } else {
                proposal.no_votes += 1;
            }

            Ok(())
        } else {
            Err("Proposal not found.".to_string())
        }
    }

    fn close_proposal(&mut self, proposal_id: u64) -> Result<(), String> {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            proposal.active = false;
            Ok(())
        } else {
            Err("Proposal not found.".to_string())
        }
    }

    fn get_proposal(&self, proposal_id: u64) -> Option<&Proposal> {
        self.proposals.get(&proposal_id)
    }
}

#[init]
fn init() -> VotingContract {
    VotingContract::new()
}

#[update]
fn create_proposal(description: String, creator: String) -> u64 {
    let mut contract = ic_cdk::storage::get_mut::<VotingContract>();
    contract.create_proposal(description, creator)
}

#[update]
fn vote(proposal_id: u64, vote: bool) -> Result<(), String> {
    let mut contract = ic_cdk::storage::get_mut::<VotingContract>();
    contract.vote(proposal_id, vote)
}

#[update]
fn close_proposal(proposal_id: u64) -> Result<(), String> {
    let mut contract = ic_cdk::storage::get_mut::<VotingContract>();
    contract.close_proposal(proposal_id)
}

#[query]
fn get_proposal(proposal_id: u64) -> Option<Proposal> {
    let contract = ic_cdk::storage::get::<VotingContract>();
    contract.get_proposal(proposal_id).cloned()
}
