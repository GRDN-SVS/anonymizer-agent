use std::error::Error;
use actix::prelude::*;

/// A message that can be sent to the DBExecutor to
/// tell it to save a nonce to the database.
pub struct SaveNonce {
    /// The actual nonce to be stored.
    pub nonce: String,
}
impl Message for SaveNonce {
    type Result = Result<(), Box<dyn Error>>;
}