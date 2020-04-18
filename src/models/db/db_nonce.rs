use crate::database::schema::nonces;

/// Representation of an Nonce already stored inside the database
#[derive(Queryable)]
pub struct Nonce {
    pub id: i32,
    /// Nonce to be used on vote encryption.
    pub nonce: String,
}

/// Struct used to insert a new Nonce on database
#[derive(Insertable)]
#[table_name = "nonces"]
pub struct InsertableNonce {
    /// Nonce to be used on vote encryption.
    pub nonce: String,   
}