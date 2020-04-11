CREATE TABLE votes (
    id SERIAL PRIMARY KEY,
    election_id SERIAL REFERENCES elections(id),
    nonce VARCHAR NOT NULL,
    encrypted_option VARCHAR NOT NULL,
    created_on TIMESTAMP NOT NULL
);