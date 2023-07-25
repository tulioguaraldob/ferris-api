use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PostsResponse {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostRequest {
    pub title: String,
    pub body: String,
    pub published: bool,
}
