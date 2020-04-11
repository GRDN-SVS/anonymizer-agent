use std::error::Error;
use actix::prelude::*;
use diesel::pg::PgConnection;

use crate::models;

/// An [actor](https://en.wikipedia.org/wiki/Actor_model) 
/// that connects to a postgres database, being the only one
/// in charge of interacting directly with it.
pub struct DBExecutor(PgConnection);

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

/// Message that can be sent to the DBExecutor to
/// tell it to save a vote to the database.
pub struct SaveVote {
    nonce: String,
    encrypted_option: String,
}
impl Message for SaveVote {
    type Result = Result<(), Box<dyn Error>>;
}
impl Handler<SaveVote> for DBExecutor {
    type Result = Result<(), Box<dyn Error>>;

    fn handle(&mut self, msg: SaveVote, _: &mut Self::Context) -> Self::Result {
        // Create vote insertion model
        // let new_vote = models::db_vote::Vote::new(election_id: i64, nonce: String, encrypted_option: String);

        // normal diesel operations
        // diesel::insert_into(nonces)
        //     .values(&new_nonce)
        //     .execute(&self.0)
        //     .expect("Error inserting nonce");

        Ok(())
    }
}