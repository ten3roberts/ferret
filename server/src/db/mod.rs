mod models;

use std::{
    env,
    path::{Path, PathBuf},
    sync::Arc,
};

use color_eyre::eyre::Context;
use eyre::ContextCompat;
use serde::{Deserialize, Serialize};
use slotmap::{new_key_type, SlotMap};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub use self::models::Post;

new_key_type! {
    pub struct PostKey;

}
#[derive(Serialize, Debug, Deserialize, Default)]
pub struct DatabaseData {
    posts: SlotMap<PostKey, Post>,
}

#[derive(Debug, Clone)]
pub struct Database {
    data: Arc<RwLock<DatabaseData>>,
}

impl Database {
    pub fn get_path() -> eyre::Result<PathBuf> {
        let mut path = dirs::data_dir().wrap_err("Unable to find data dir")?;
        path.push("ferret");
        path.push("db.json");
        Ok(path)
    }

    pub async fn open() -> eyre::Result<Self> {
        let data = match tokio::fs::read_to_string(Self::get_path()?).await {
            Ok(s) => serde_json::from_str(&s).wrap_err("Failed to read db from json")?,
            Err(_) => DatabaseData::default(),
        };

        Ok(Self {
            data: Arc::new(RwLock::new(data)),
        })
    }

    pub async fn save(&self) -> eyre::Result<()> {
        self.read().await.save().await
    }

    pub async fn read<'a>(&'a self) -> RwLockReadGuard<'a, DatabaseData> {
        self.data.read().await
    }
    pub async fn write<'a>(&'a self) -> RwLockWriteGuard<'a, DatabaseData> {
        self.data.write().await
    }

    pub async fn add_post(&self, post: Post) -> PostKey {
        self.data.write().await.posts.insert(post)
    }
}

impl DatabaseData {
    pub async fn save(&self) -> eyre::Result<()> {
        let s = serde_json::to_string(self)?;
        let path = Database::get_path()?;
        tokio::fs::write(path, &s)
            .await
            .wrap_err("Failed to save db")
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let data = self.data.clone();
        tokio::task::spawn(async move { data.read().await.save().await });
    }
}
