use serde::{Serialize, Deserialize};

/// Representation of a nonce
#[derive(Serialize, Deserialize)]
pub struct Nonce {
    /// The id of the generated nonce
    pub id: i32,
    /// The nonce text
    pub nonce: String,
    
}