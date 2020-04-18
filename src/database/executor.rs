extern crate textnonce;
use std::error::Error;
use actix::prelude::*;
use diesel::pg::PgConnection;
use diesel::query_dsl::RunQueryDsl;
use crate::models;

use super::schema::nonces;

/// An [actor](https://en.wikipedia.org/wiki/Actor_model) 
/// that connects to a postgres database, being the only one
/// in charge of interacting directly with it.
pub struct DBExecutor(PgConnection);

impl Actor for DBExecutor {
    type Context = SyncContext<Self>;
}

/// Message that can be sent to the DBExecutor to
/// tell it to save a nonce to the database.
pub struct SaveNonce {
    id: i32,
    nonce,
}

impl Message for SaveNonce {
    type Result = Result<(), Box<dyn Error>>;
}
impl Handler<SaveNonce> for DBExecutor {
    type Result = Result<(), Box<dyn Error>>;

    fn handle(&mut self, msg: SaveNonce, _: &mut Self::Context) -> Self::Result {
        // Create nonce insertion model
        let nonce = textnonce::TextNonce::sized_urlsafe(32)
            .unwrap()
            .into_string();

        let insertable_nonce = models::db::InsertableNonce { nonce };
        
        diesel::insert_into(nonces::table)
            .values(&insertable_nonce)
            .execute(&self.0)?;

        Ok(())
    }
}