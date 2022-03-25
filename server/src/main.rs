use serde_json::json;
pub use server::*;

use axum::{
    body::Body,
    extract::{Extension, Form},
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use server::db::{Database, Post};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Running server");

    let db = Database::open().await?;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
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
    Form(CreatePost { title, body }): Form<CreatePost>,
) -> impl IntoResponse {
    let post = Post {
        user: "user".to_string(),
        title,
        body,
    };
    tracing::info!("Created post: {:?}", post);

    let key = db.add_post(post).await;
    "Post created"
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, Internet!"
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
