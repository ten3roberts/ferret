use hyper::Body;
use reqwest::StatusCode;
pub use server::*;
mod auth;

use axum::{
    body::Bytes,
    extract::{Extension, Path},
    http::{Request, Response},
    middleware::{self, Next},
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
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "server=debug,tower_http=debug")
    }
    tracing::info!("Running server");

    let db = Database::open()?;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        // .layer(middleware::from_fn(print_request_response))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(middleware::from_fn(print_request_response))
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
        user_id: &claims.sub,
        title: &title,
        body: &body,
    };

    let post = serde_json::to_string(&db.create_post(post, &claims).await?).unwrap();

    tracing::info!("User {claims:#?} created new post: {post:#?}");

    Ok::<_, db::Error>(Json(post))
}

async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes> + Sized,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{} body = {:?}", direction, body);
    }

    Ok(bytes)
}
