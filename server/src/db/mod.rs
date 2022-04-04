pub mod models;

use std::{env, sync::Arc};

use self::models::NewPost;
pub use self::models::*;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use thiserror::Error;
use tokio::sync::Mutex;

use diesel::pg::*;
use diesel::prelude::*;
use dotenv::dotenv;

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<PgConnection>>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0:?}")]
    DbError(#[from] diesel::result::Error),
    #[error("Post cannot be empty")]
    EmptyPost,
    #[error("Failed to connect to database")]
    ConnectionError(#[from] diesel::result::ConnectionError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Reason: {self}")).into_response()
    }
}

impl Database {
    pub fn open() -> Result<Self> {
        dotenv().unwrap();
        let db_url = env::var("DATABASE_URL").unwrap();
        Ok(Self {
            conn: Arc::new(Mutex::new(PgConnection::establish(&db_url)?)),
        })
    }

    pub async fn create_post<'a>(&self, post: NewPost<'a>) -> Result<Post> {
        if post.body.is_empty() || post.title.is_empty() || post.username.is_empty() {
            tracing::warn!("Post cannot be empty");
            return Err(Error::EmptyPost);
        }
        diesel::insert_into(crate::schema::posts::table)
            .values(&post)
            .get_result(&*self.conn.lock().await)
            .map_err(|e| e.into())
    }

    pub async fn get_user_posts(&self, user: &str) -> Result<Vec<Post>> {
        use crate::schema::posts::dsl::*;
        posts
            .filter(username.eq_all(user))
            .load(&*self.conn.lock().await)
            .map_err(|e| e.into())
    }

    pub async fn get_top_posts(&self, limit: i64) -> Result<Vec<Post>> {
        use crate::schema::posts::dsl::*;
        posts
            .order(created_at.desc())
            .limit(limit)
            .load(&*self.conn.lock().await)
            .map_err(|e| e.into())
    }

    pub async fn get_post(&self, key: i32) -> Result<Post> {
        use crate::schema::posts::dsl::*;
        posts
            .filter(id.eq_all(key))
            .first(&*self.conn.lock().await)
            .map_err(|e| e.into())
    }
}

// #[derive(Serialize, Debug, Deserialize, Default)]
// pub struct DatabaseData {
//     posts: DashMap<Uuid, Post>,
// }

// #[derive(Debug, Clone)]
// pub struct Database {
//     data: Arc<RwLock<DatabaseData>>,
//     events: Sender<DbEvent>,
// }

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub enum DbEvent {
//     CreatePost(Uuid, Post),
// }

// impl Database {
//     pub fn get_path() -> eyre::Result<PathBuf> {
//         let mut path = dirs::data_dir().wrap_err("Unable to find data dir")?;
//         path.push("ferret");
//         path.push("db.json");
//         Ok(path)
//     }
//     pub fn get_event_path() -> eyre::Result<PathBuf> {
//         let mut path = dirs::data_dir().wrap_err("Unable to find data dir")?;
//         path.push("ferret");
//         path.push("db_events.json");
//         Ok(path)
//     }

//     pub async fn open() -> eyre::Result<Self> {
//         let data = match tokio::fs::read_to_string(Self::get_path()?).await {
//             Ok(s) => serde_json::from_str(&s).wrap_err("Failed to read db from json")?,
//             Err(_) => DatabaseData::default(),
//         };

//         let events_file = OpenOptions::new()
//             .create(true)
//             .read(true)
//             .open(Self::get_event_path().unwrap())
//             .await?;

//         // let reader = BufReader::new(events_file);
//         // data.append(reader.lines().map())

//         let (tx, rx) = flume::bounded(128);
//         let db = Self {
//             data: Arc::new(RwLock::new(data)),
//             events: tx,
//         };

//         tokio::spawn(async move { while let Ok(e) = rx.recv_async().await {} });

//         Ok(db)
//     }

//     pub async fn save(&self) -> eyre::Result<()> {
//         self.read().await.save().await
//     }

//     pub async fn read<'a>(&'a self) -> RwLockReadGuard<'a, DatabaseData> {
//         self.data.read().await
//     }
//     pub async fn write<'a>(&'a self) -> RwLockWriteGuard<'a, DatabaseData> {
//         self.data.write().await
//     }

//     pub async fn add_post(&self, post: Post) -> Uuid {
//         let id = Uuid::new_v4();
//         self.data.write().await.posts.insert(id, post);
//         id
//     }
// }

// impl DatabaseData {
//     pub async fn save(&self) -> eyre::Result<()> {
//         let s = serde_json::to_string(self)?;
//         let path = Database::get_path()?;
//         tokio::fs::write(path, &s)
//             .await
//             .wrap_err("Failed to save db")
//     }

//     pub fn append(&mut self, events: impl Iterator<Item = DbEvent>) {
//         for event in events {
//             match event {
//                 DbEvent::CreatePost(id, val) => {
//                     self.posts.insert(id, val);
//                 }
//             }
//         }
//     }
// }

// impl Drop for Database {
//     fn drop(&mut self) {
//         let data = self.data.clone();
//         tokio::task::spawn(async move { data.read().await.save().await });
//     }
// }
