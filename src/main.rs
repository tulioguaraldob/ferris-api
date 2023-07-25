pub mod model;

use std::ptr::null;

use mysql::prelude::*;
use mysql::*;

fn dsn() -> Result<String, Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let mysql_user = std::env::var("MYSQL_USER")?;
    let mysql_password = std::env::var("MYSQL_PASSWORD")?;
    let mysql_host = std::env::var("MYSQL_HOST")?;
    let mysql_port = std::env::var("MYSQL_PORT")?;
    let mysql_database = std::env::var("MYSQL_DATABASE")?;

    return Ok(format!("mysql://{mysql_user}:{mysql_password}@{mysql_host}:{mysql_port}/{mysql_database}"));
}

fn open_connection() -> std::result::Result<mysql::PooledConn, Box<dyn std::error::Error>> {
    let pool = Pool::new(dsn()?.as_str())?;
    
    return Ok(pool.get_conn()?);
}

// CREATE TABLE IF NOT EXISTS users (id bigint primary key auto_increment, name varchar(255) not null);
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut db = open_connection()?;
    let users = db.query_map(r"
    SELECT * FROM users
    ", |(id, name)| {
        model::user::User { id, name }
    })?;

    for user in users {
        println!("/ - / - / - / - / -");
        println!("ID: {}", user.id);
        println!("-*-*-*-*-*");
        println!("-*-*-*-*-*");
        println!("Name: {}", user.name);
        println!("-*-*-*-*-*");
        println!("-*-*-*-*-*");
        println!("/ - / - / - / - / -");
    }

    return Ok(());
}
