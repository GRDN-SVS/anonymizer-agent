/// Representation of an arbitrary number that can be
/// used just once in a cryptographic communication
pub struct Nonce {
    /// An Id used to identify the nonce inside the database
    pub id: String,
    /// The actual arbitrary number
    pub nonce: String,
}

impl Nonce {
    /// Returns a new `Nonce` instance
    /// 
    /// # Arguments 
    /// * `id` - A String containing the unique id to be used during storage
    /// * `nonce` - A String containing the actual nonce to be used
    pub fn new(id: String, nonce: String) -> Nonce {
        Nonce { id, nonce }
    }
}