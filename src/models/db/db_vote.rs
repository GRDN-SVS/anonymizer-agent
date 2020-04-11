use crate::database::schema::votes;

/// Representation of a Vote already stored inside the database
#[derive(Queryable)]
pub struct Vote {
    pub id: i32,
    /// Id of the election this vote is associated with
    pub election_id: i32,
    /// Unique arbitrary number used to encrypt the contents of the vote
    pub nonce: String,
    /// Encrypted contents of the vote
    pub encrypted_option: String,
    /// Timestamp indicating the moment in which the entry was created
    pub created_on: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "votes"]
pub struct InsertableVote {
    // The id of the election this vote is associated with
    pub election_id: i32,
    /// Unique arbitrary number used to encrypt the contents of the vote
    pub nonce: String,
    /// Encrypted contents of the vote
    pub encrypted_option: String,
    /// Timestamp indicating the moment in which the entry was created
    pub created_on: chrono::NaiveDateTime,
}