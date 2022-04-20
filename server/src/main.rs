use serde_json::json;
pub use server::*;
mod auth;

use axum::{
    extract::{Extension, Path},
    headers::Cookie,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use server::{auth::Claims, db::Database};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Running server");

    let db = Database::open()?;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/create_post", post(create_post))
        .route("/posts", get(get_posts))
        .route("/post/:id", get(get_post))
        .layer(Extension(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 13000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub async fn get_post(Path(id): Path<i32>, db: Extension<Database>) -> impl IntoResponse {
    tracing::info!("Retrieving post: {id}");
    let posts = serde_json::to_string(&db.get_post(id).await?).unwrap();

    Ok::<_, db::Error>(posts)
}

pub async fn get_posts(db: Extension<Database>) -> impl IntoResponse {
    tracing::info!("Posts");
    let posts = serde_json::to_string(&db.get_top_posts(20).await?).unwrap();
    tracing::info!("Posts: {posts}");

    Ok::<_, db::Error>(posts)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
}

async fn create_post(
    db: Extension<Database>,
    Json(CreatePost { title, body }): Json<CreatePost>,
    claims: Claims,
) -> impl IntoResponse {
    tracing::info!("Creating post. Claims: {claims}");
    let post = db::models::NewPost {
        username: "user",
        title: &title,
        body: &body,
    };

    dbg!(&post);

    let post = serde_json::to_string(&db.create_post(post).await?).unwrap();
    tracing::info!("Post: {:?}", post);

    Ok::<_, db::Error>(Json(post))
}
