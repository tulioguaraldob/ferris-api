pub mod schema;

pub mod dto;
pub mod models;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::post::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn show_posts() -> Vec<Post> {
    use self::schema::posts::dsl::*;
    let database_connection = &mut establish_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(database_connection)
        .expect("Error loading posts");

    return results;
}

pub fn create_post(post: models::post::NewPost) {
    let database_connection = &mut establish_connection();

    diesel::insert_into(schema::posts::table)
        .values(&post)
        .returning(Post::as_returning())
        .get_result(database_connection)
        .expect("Failed to create post");
}
