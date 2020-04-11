use serde::{Serialize, Deserialize};

/// Representation of a vote
#[derive(Serialize, Deserialize)]
pub struct Vote {
    /// The id of the election the vote takes place in
    pub election_id: i32,
    /// The id of the voter owning this vote
    pub voter_id: i32,
    /// The id of the option chosen by the voter
    pub option: i32,
}