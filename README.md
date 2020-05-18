# Anonymizer Agent

The Anonymizer agent is an independent compute unit inside the GRDN
system. It is in charge of receiving HTTP requests, creating a nonce,
storing it inside a database, and returning said nonce to the caller.

## Built With

  * [**actix-web**](https://actix.rs/) framework to manage web
    interaction.
  * [**diesel**](https://diesel.rs/) object relational mapper in order
    to interact with the PostgreSQL database.

## Dev Dependencies
libpq-dev (postgres)
diesel_cli (optional)
