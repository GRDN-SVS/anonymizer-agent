table! {
    elections (id) {
        id -> Int4,
        init_date -> Timestamp,
        end_date -> Timestamp,
        created_on -> Timestamp,
    }
}

table! {
    votes (id) {
        id -> Int4,
        election_id -> Int4,
        nonce -> Varchar,
        encrypted_option -> Varchar,
        created_on -> Timestamp,
    }
}

joinable!(votes -> elections (election_id));

allow_tables_to_appear_in_same_query!(
    elections,
    votes,
);
