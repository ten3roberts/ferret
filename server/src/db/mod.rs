pub mod models;

use std::{env, sync::Arc};

use crate::auth::Claims;
use crate::schema;
use crate::schema::users;

use self::models::NewPost;
pub use self::models::*;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Redirect;
use diesel::associations::HasTable;
use thiserror::Error;
use tokio::sync::Mutex;

use diesel::pg::*;
use diesel::prelude::*;
use dotenv::dotenv;
use tracing::error;
use tracing::info;

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
    #[error("User is not authorized for this action")]
    Unauthorized,
}

pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Error::DbError(diesel::result::Error::NotFound) => StatusCode::NOT_FOUND,
            Error::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::EmptyPost => StatusCode::BAD_REQUEST,
            Error::ConnectionError(_) => StatusCode::BAD_GATEWAY,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
        };

        (status, format!("Reason: {self}")).into_response()
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

    pub async fn get_init_user(&self, claims: &Claims) -> Result<User> {
        use super::schema::users::dsl::*;
        let conn = self.conn.lock().await;
        // Create or update user
        let user: User = diesel::insert_into(crate::schema::users::table)
            .values(&User {
                user_id: claims.sub.clone(),
                picture: claims.picture.to_owned(),
                name: claims.username.to_owned(),
            })
            .on_conflict(user_id)
            .do_update()
            .set((
                name.eq_all(&claims.username),
                picture.eq_all(&claims.picture),
            ))
            .get_result(&*conn)
            .map_err(|e| {
                error!("{e}");
                e
            })?;

        Ok(user)
    }

    pub async fn find_posts(&self, text: &str) -> Result<Vec<UserPost>> {
        const PAGE_COUNT: i64 = 64;

        let conn = self.conn.lock().await;
        let text = text.to_lowercase();

        use crate::schema::posts::dsl::*;
        use crate::schema::users::dsl::*;
        let mut result = Vec::new();

        for page in 0.. {
            let res = posts
                .offset(page * PAGE_COUNT)
                .limit(PAGE_COUNT)
                .load::<Post>(&*conn)?
                .into_iter()
                .filter(|post| {
                    post.title.to_lowercase().contains(&text)
                        || post.body.to_lowercase().contains(&text)
                })
                .map(|post| -> Result<_> {
                    let user = users.find(&post.user_id).first(&*conn)?;
                    Ok(UserPost::new(user, post, vec![]))
                })
                .flatten();

            let old_size = result.len();
            result.extend(res);
            if result.len() == old_size {
                break;
            }
        }

        Ok(result)
    }

    pub async fn create_comment(&self, comment: &NewComment, claims: &Claims) -> Result<Comment> {
        use crate::schema::comments::{self, *};
        let user: User = self.get_init_user(claims).await?;

        let comment: Comment = diesel::insert_into(comments::table)
            .values(&(
                post_id.eq_all(comment.post_id),
                user_id.eq_all(user.user_id),
                body.eq_all(&comment.body),
            ))
            .get_result(&*self.conn.lock().await)?;

        Ok(comment)
    }

    pub async fn create_post(&self, post: &NewPost, claims: &Claims) -> Result<UserPost> {
        if post.title.is_empty() {
            tracing::warn!("Post cannot be empty");
            return Err(Error::EmptyPost);
        }

        use crate::schema::posts::*;

        tracing::info!("Inserting post {post:?}");

        let user = self.get_init_user(claims).await?;
        info!("User: {user:#?}");

        let post: Post = diesel::insert_into(crate::schema::posts::table)
            .values(&(
                user_id.eq_all(&user.user_id),
                title.eq_all(&post.title),
                body.eq_all(&post.body),
            ))
            .get_result(&*self.conn.lock().await)?;

        Ok(UserPost::new(user, post, vec![]))
    }

    pub async fn get_user_posts(
        &self,
        user: &str,
        limit: i64,
        offset: i64,
    ) -> Result<(User, Vec<Post>)> {
        use crate::schema::posts::dsl::*;
        use crate::schema::users::dsl::*;

        let conn = self.conn.lock().await;
        let user: User = users.find(user).first(&*conn).unwrap();
        let res: Vec<Post> = Post::belonging_to(&user)
            .order(created_at.desc())
            .limit(limit)
            .load(&*conn)
            .unwrap();
        Ok((user, res))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_top_posts(&self, limit: i64) -> Result<Vec<UserPost>> {
        use crate::schema::posts;
        use crate::schema::posts::*;
        let res: Vec<(User, Post)> = users::table
            .inner_join(posts::table)
            .order(created_at.desc())
            .limit(limit)
            .load(&*self.conn.lock().await)?;

        Ok(res
            .into_iter()
            .map(|(user, post)| UserPost::new(user, post, vec![]))
            .collect())
    }

    pub async fn get_comments(&self, post: i32) -> Result<Vec<UserComment>> {
        use crate::schema::comments::dsl::*;
        use crate::schema::*;

        let result: Vec<UserComment> = comments
            .filter(post_id.eq(post))
            .order(created_at.asc())
            .inner_join(users::table)
            .load::<(Comment, User)>(&*self.conn.lock().await)?
            .into_iter()
            .map(|(comment, user)| UserComment { comment, user })
            .collect();

        Ok(result)
    }

    #[tracing::instrument(skip(self, claims))]
    pub async fn delete_post(&self, id: i32, claims: Claims) -> Result<Redirect> {
        tracing::info!("Deleting post");
        let conn = self.conn.lock().await;

        use schema::posts::dsl::*;
        use schema::users::dsl::*;

        let comment: Post = posts.find(id).first(&*conn)?;
        let user: User = users.find(&comment.user_id).first(&*conn)?;

        if user.user_id != claims.sub {
            return Err(Error::Unauthorized);
        }

        diesel::delete(posts.find(id)).execute(&*conn).unwrap();
        tracing::info!("Here");

        dbg!(Ok(Redirect::to("/")))
    }

    pub async fn delete_comment(&self, id: i32, claims: Claims) -> Result<Redirect> {
        use schema::comments::dsl::*;

        let comment: Comment = comments.find(id).first(&*self.conn.lock().await)?;
        let user: User = users
            .find(&comment.user_id)
            .first(&*self.conn.lock().await)?;

        use schema::users::dsl::*;

        self.authorize(&claims, user).await?;

        diesel::delete(comments.find(id)).execute(&*self.conn.lock().await)?;

        Ok(Redirect::to(&(format!("/post/{}", comment.post_id))))
    }

    pub async fn authorize(&self, claims: &Claims, owner: User) -> Result<()> {
        if owner.user_id != claims.sub {
            Err(Error::Unauthorized)
        } else {
            Ok(())
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_post(&self, key: i32) -> Result<UserPost> {
        use crate::schema::posts::dsl::*;
        let conn = self.conn.lock().await;
        let post: Post = posts.find(key).first(&*conn)?;

        let user: User = crate::schema::users::table
            .find(&post.user_id)
            .first(&*conn)
            .unwrap();

        drop(conn);

        let comments = self.get_comments(post.post_id).await?;

        use crate::schema::solved_metas::dsl::*;
        let solved: Option<SolvedMeta> = match solved_metas
            .find(post.post_id)
            .first(&*self.conn.lock().await)
        {
            Ok(comment) => Some(comment),
            Err(diesel::result::Error::NotFound) => None,
            Err(e) => return Err(e.into()),
        };

        Ok(UserPost::new(user, post, comments).solved_by(solved))
    }

    pub async fn mark_solved(&self, claims: &Claims, post: i32, comment: i32) -> Result<()> {
        use crate::schema::posts::dsl::posts;
        use crate::schema::solved_metas;
        use crate::schema::solved_metas::*;
        use crate::schema::users::dsl::users;
        let subj: Post = posts.find(post).first(&*self.conn.lock().await)?;
        let owner = users.find(subj.user_id).first(&*self.conn.lock().await)?;
        self.authorize(claims, owner).await?;

        diesel::insert_into(solved_metas::table)
            .values(&SolvedMeta {
                post_id: post,
                comment_id: comment,
            })
            .on_conflict(post_id)
            .do_update()
            .set(comment_id.eq(comment))
            .execute(&*self.conn.lock().await)?;

        Ok(())
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
