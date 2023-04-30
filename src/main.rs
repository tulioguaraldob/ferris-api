use postgres::{Client, Error, NoTls};
use std::collections::HashMap;
mod models;
use models::author;

fn migration(postgres_client: &mut Client) {
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

fn main() -> Result<(), Error> {
    let mut postgres_client = Client::connect("postgresql://teste:teste@localhost/teste", NoTls)?;
    migration(&mut postgres_client);

    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindranath Tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (key, value) in &authors {
        let author = author::Author {
            id: 0,
            name: key.to_string(),
            country: value.to_string(),
        };

        postgres_client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
    }

    Ok(())
}
