use postgres::{Client, Error, NoTls};
use std::collections::HashMap;
mod models;
use models::author;
use models::db;

//  docker run -p '5432:5432' -e POSTGRES_PASSWORD=teste -e POSTGRES_USER=teste -e POSTGRES_DB=teste -d postgres

fn main() -> Result<(), Error> {
    let mut postgres = db::open_connection()?;
    db::migration(&mut postgres);

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

        postgres.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
    }

    Ok(())
}
