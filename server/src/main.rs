use serde_json::json;
pub use server::*;

use axum::{
    body::{Body, HttpBody},
    extract::{Extension, Form},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use server::db::Database;
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
        .layer(Extension(db));
    // `POST /users` goes to `create_user`

    let addr = SocketAddr::from(([127, 0, 0, 1], 13000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
}

async fn create_post(
    db: Extension<Database>,
    Json(CreatePost { title, body }): Json<CreatePost>,
) -> impl IntoResponse {
    let post = db::models::NewPost {
        username: "user",
        title: &title,
        body: &body,
    };

    let post = db.create_post(post).await?;
    tracing::info!("Post: {:?}", post);
    Ok::<&'static str, db::Error>("Post created")
}
