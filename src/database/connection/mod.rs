pub mod messages;

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

impl Handler<messages::SaveNonce> for DBExecutor {
    type Result = Result<(), Box<dyn Error>>;

    fn handle(&mut self, msg: messages::SaveNonce, _: &mut Self::Context) -> Self::Result {
        // Create nonce insertion model
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_nonce = models::Nonce::new(uuid, msg.nonce);

        // normal diesel operations
        // diesel::insert_into(nonces)
        //     .values(&new_nonce)
        //     .execute(&self.0)
        //     .expect("Error inserting nonce");

        Ok(())
    }
}