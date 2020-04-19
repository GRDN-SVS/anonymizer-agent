extern crate textnonce;
use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::query_dsl::RunQueryDsl;
use diesel::Connection;

use crate::models;

use super::schema::nonces;

/// An [actor](https://en.wikipedia.org/wiki/Actor_model)
/// that connects to a postgres database, being the only one
/// in charge of interacting directly with it.
pub struct DBExecutor(PgConnection);

impl DBExecutor {
    pub fn new(connection_string: &str) -> DBExecutor {
        DBExecutor(
            diesel::pg::PgConnection::establish(connection_string)
                .expect("Could not Connect to the database"),
        )
    }
}

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

/// Message that can be sent to the DBExecutor to
/// tell it to save a nonce to the database.
pub struct SaveNonce {
    pub nonce: String,
}

impl Message for SaveNonce {
    type Result = Result<models::Nonce, diesel::result::Error>;
}

impl Handler<SaveNonce> for DBExecutor {
    type Result = Result<models::Nonce, diesel::result::Error>;

    fn handle(&mut self, msg: SaveNonce, _: &mut Self::Context) -> Self::Result {
        let insertable_nonce = models::InsertableNonce { nonce: msg.nonce };

        let nonce_obj = diesel::insert_into(nonces::table)
            .values(&insertable_nonce)
            .get_result(&self.0)?;

        Ok(nonce_obj)
    }
}
