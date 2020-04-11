# Anonymizer Agent

The Anonymizer agent is an independent compute unit inside the GRDN
system. It is in charge of receiving votes through HTTP requests from
the anonymity network, associating a nonce to each of them, encrypting
its contents, storing the new encrypted vote inside a database, and then
signling the Judge agent that the vote is ready to be examined.

## Built With

  * [**actix-web**](https://actix.rs/) framework to manage web
    interaction.
  * [**diesel**](https://diesel.rs/) object relational mapper in order
    to interact with the PostreSQL database.
  * [**sodiumoxide**](https://github.com/sodiumoxide/sodiumoxide)
    cryptography library to encrypt the contents of the votes.

## Dev Dependencies
libpq-dev (postgres)
diesel_cli (optional)