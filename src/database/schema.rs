table! {
    nonces (id) {
        id -> Int4,
        nonce -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    nonces,
);
