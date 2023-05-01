use postgres::{Client, NoTls};

pub fn open_connection() -> Result<Client, postgres::Error> {
    let mut postgres_client = Client::connect("postgresql://teste:teste@localhost/teste", NoTls);
    return postgres_client;
}

pub fn migration(postgres_client: &mut Client) {
    let authorCreationQuery = "CREATE TABLE IF NOT EXISTS author (
        id              SERIAL PRIMARY KEY,
        name            VARCHAR NOT NULL,
        country         VARCHAR NOT NULL
        )";

    let bookCreationQuery = "CREATE TABLE IF NOT EXISTS book  (
        id              SERIAL PRIMARY KEY,
        title           VARCHAR NOT NULL,
        author_id       INTEGER NOT NULL REFERENCES author
        )";

    postgres_client.batch_execute(authorCreationQuery);
    postgres_client.batch_execute(bookCreationQuery);
}
