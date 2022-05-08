use color_eyre::Report;
use eyre::Context;
use hyper::Body;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
pub use server::*;
mod auth;

use axum::{
    body::Bytes,
    extract::{Extension, Path, Query},
    http::{Request, Response},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use server::{
    auth::Claims,
    db::{models::NewPost, Database, NewComment, SolvedMeta},
};
use std::net::SocketAddr;
use tracing::instrument;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Registry,
};

#[tokio::main]
#[instrument]
async fn main() -> color_eyre::Result<()> {
    // initialize tracing
    color_eyre::install()?;
    Registry::default()
        .with(tracing_tree::HierarchicalLayer::new(2))
        .with(ErrorLayer::default())
        .init();

    tracing::info!("Running server");

    let db = Database::open()?;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .route("/post/:id", delete(delete_post))
        .route("/comment/:id", delete(delete_comment))
        .route("/create_post", post(create_post))
        .route("/create_comment", post(create_comment))
        .route("/user/:id", get(get_user))
        .route("/posts", get(get_posts))
        .route("/post/:id", get(get_post))
        .route("/post/mark_solved", patch(mark_solved))
        .route("/search/", get(find_posts))
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

pub async fn get_user(db: Extension<Database>, Path(user_id): Path<String>) -> impl IntoResponse {
    tracing::info!("Posts");
    let (user, posts) = db.get_user_posts(&user_id, 20, 0).await?;

    Ok::<_, db::Error>(Json(json! (
        {
            "user": user,
            "posts": posts,
        }
    )))
}

pub async fn get_posts(db: Extension<Database>) -> impl IntoResponse {
    tracing::info!("Posts");
    let posts = db.get_top_posts(20).await?;

    Ok::<_, db::Error>(Json(posts))
}

async fn create_comment(
    db: Extension<Database>,
    Json(comment): Json<NewComment>,
    claims: Claims,
) -> impl IntoResponse {
    tracing::info!("Creating post. Claims: {claims}");

    let post = db.create_comment(&comment, &claims).await?;

    tracing::info!("User {claims:#?} created new post: {post:#?}");

    Ok::<_, db::Error>(Json(post))
}

async fn create_post(
    db: Extension<Database>,
    Json(post): Json<NewPost>,
    claims: Claims,
) -> impl IntoResponse {
    tracing::info!("Creating post. Claims: {claims}");

    let post = serde_json::to_string(&db.create_post(&post, &claims).await?).unwrap();

    tracing::info!("User {claims:#?} created new post: {post:#?}");

    Ok::<_, db::Error>(Json(post))
}

async fn delete_post(
    db: Extension<Database>,
    Path(id): Path<i32>,
    claims: Claims,
) -> impl IntoResponse {
    tracing::info!("Deleting post: {id}");
    let res = db.delete_post(id, claims).await;
    if let Err(err) = &res {
        tracing::warn!("Failed to delete post: {err}")
    }

    res
}

async fn delete_comment(
    db: Extension<Database>,
    Path(id): Path<i32>,
    claims: Claims,
) -> impl IntoResponse {
    tracing::info!("Deleting comment: {id}");
    let res = db.delete_comment(id, claims).await;
    if let Err(err) = &res {
        tracing::warn!("Failed to delete post: {err}")
    }
    res
}

async fn mark_solved(
    db: Extension<Database>,
    Json(solved): Json<SolvedMeta>,
    claims: Claims,
) -> impl IntoResponse {
    let res = db
        .mark_solved(&claims, solved.post_id, solved.comment_id)
        .await;

    if let Err(err) = &res {
        tracing::warn!("Failed to mark solved: {err}")
    }

    tracing::info!("Marked post as solved: {res:?}");

    res
}

#[derive(Debug, Serialize, Deserialize)]
struct FindParams {
    text: String,
}
async fn find_posts(
    db: Extension<Database>,
    Query(params): Query<FindParams>,
) -> impl IntoResponse {
    let posts = db.find_posts(&params.text).await?;
    Ok::<_, db::Error>(Json(json!({ "posts": posts })))
}
