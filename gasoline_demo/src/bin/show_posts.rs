use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use gasoline_demo::*;

async fn get_all_posts() -> Result<impl Responder> {
    let results = show_posts();
    let mut posts_response: Vec<dto::posts::PostsResponse> = Vec::new();
    for result in results {
        let post_response: dto::posts::PostsResponse = dto::posts::PostsResponse {
            id: result.id,
            title: result.title,
            body: result.body,
            published: result.published,
        };

        posts_response.push(post_response);
    }

    return Ok(HttpResponse::Ok().json(&posts_response));
}

async fn register_post(post_request: web::Json<dto::posts::PostRequest>) -> Result<impl Responder> {
    let post: models::post::NewPost = models::post::NewPost {
        title: post_request.clone().title,
        body: post_request.clone().body,
        published: post_request.published,
    };

    println!("{}", post.body);
    create_post(post);

    return Ok(HttpResponse::Created().json("post created successfully"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route("/post", web::get().to(get_all_posts))
                .route("/post", web::post().to(register_post))
                .route("/health", web::get().to(health_check)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn health_check() -> impl Responder {
    return "Healthy as a bull";
}
