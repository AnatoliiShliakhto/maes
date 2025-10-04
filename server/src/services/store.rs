use crate::services::*;
use ::moka::future::Cache;
use ::serde::{Deserialize, Serialize};
use ::shared::{common::*, services::*, utils::*};
use ::std::{
    any::Any,
    collections::HashSet,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};
use ::tokio::{
    fs,
    io::AsyncWriteExt,
    sync::{OnceCell, RwLock},
    task::spawn_blocking,
};

static CIPHER: LazyLock<Arc<Cipher>> = LazyLock::new(|| {
    Arc::new(Cipher::init().unwrap_or_else(|e| panic!("Cipher init failed: {e}")))
});
static PATH: OnceCell<Arc<PathBuf>> = OnceCell::const_new();
static CACHE: LazyLock<Cache<String, Arc<dyn Any + Send + Sync>>> =
    LazyLock::new(|| Cache::builder().max_capacity(1_000).build());

#[derive(Copy, Clone)]
pub struct Store;

impl Store {
    pub async fn init(path: impl Into<PathBuf>) -> Result<()> {
        let path = path.into().join("workspaces");
        fs::create_dir_all(&path).await.map_err(map_log_err)?;
        PATH.set(Arc::new(path.clone())).map_err(map_log_err)?;
        Ok(())
    }

    pub async fn find<T: Cachable + for<'de> Deserialize<'de> + 'static>(
        workspace: impl Into<String>,
        id: impl Into<String>,
    ) -> Result<Arc<RwLock<T>>> {
        let ws_id = workspace.into();
        let id = id.into();
        let cache_id = format!("{ws_id}{id}");

        if let Some(erased) = CACHE.get(&cache_id).await
            && let Some(arc) = erased.downcast_ref::<Arc<RwLock<T>>>()
        {
            return Ok(arc.clone());
        }

        let path = Self::get_path(&ws_id, &id).map_err(map_log_err)?;

        let erased = CACHE
            .try_get_with(cache_id.clone(), async move {
                let data = fs::read(path).await.map_err(map_log_err)?;
                let val = Self::decrypt_binary::<T>(ws_id, data, false)
                    .await
                    .map_err(map_log_err)?;
                let arc: Arc<Arc<RwLock<T>>> = Arc::new(Arc::new(RwLock::new(val)));
                Ok::<Arc<dyn Any + Send + Sync>, Error>(arc)
            })
            .await.map_err(|_| (StatusCode::NOT_FOUND, "file-not-found"))?;

        let arc = erased
            .downcast_ref::<Arc<RwLock<T>>>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "type-mismatch"))?
            .clone();

        Ok(arc)
    }

    pub async fn upsert<T: Cachable + Serialize + Clone + Send + 'static>(
        payload: T,
    ) -> Result<()> {
        let ws_id = payload.get_ws();
        let id = payload.get_id();
        let cache_id = format!("{ws_id}{id}");
        let path = Self::get_path(&ws_id, &id)?;

        if !CACHE.contains_key(&cache_id) {
            put_cached(cache_id, payload.clone()).await;
        }

        tokio::spawn(async move {
            if let Ok(data) = Self::encrypt_binary::<T>(ws_id, payload, false)
                .await
                .map_err(map_log_err)
            {
                save_atomic(path, &data).await.map_err(map_log_err).ok();
            }
        });
        Ok(())
    }

    pub async fn delete(workspace: impl AsRef<str>, id: impl AsRef<str>) -> Result<()> {
        let ws_id = workspace.as_ref();
        let id = id.as_ref();
        let cache_id = format!("{ws_id}{id}");

        if let Ok(path) = Self::get_path(ws_id, id) {
            fs::remove_file(path).await.ok();
        }
        pop_cached(cache_id).await;
        Ok(())
    }

    pub async fn batch_remove(workspace: impl Into<String>, ids: Vec<String>) -> Result<()> {
        let ws_id = workspace.into();
        let ids = ids.into_iter().collect::<HashSet<String>>();

        tokio::spawn(async move {
            for id in ids {
                Self::delete(&ws_id, id).await.ok();
            }
        });
        Ok(())
    }

    pub async fn remove_workspace(workspace: impl Into<String>) -> Result<()> {
        let ws_id = workspace.into();
        tokio::spawn(async move {
            if let Some(path) = PATH.get().map(|p| p.join(&ws_id)) && path.exists() {
                if let Ok(mut dir) = fs::read_dir(&path).await {
                    while let Ok(Some(entry)) = dir.next_entry().await {
                        let Ok(filename) = entry.file_name().into_string() else { continue };
                        pop_cached(filename.trim_end_matches(".bin")).await;
                    };
                }
                fs::remove_dir_all(path).await.map_err(map_log_err).ok();
            }
            pop_cached(&ws_id).await;
        });
        Ok(())
    }

    pub async fn encrypt_json<T: Serialize + Send + 'static>(
        workspace: impl Into<String>,
        data: T,
    ) -> Result<String> {
        let ws_id = workspace.into();
        spawn_blocking(move || -> Result<String> {
            CIPHER.get(ws_id)?.encrypt_json::<T>(data)
        })
            .await
            .map_err(map_log_err)?
    }

    pub async fn encrypt_binary<T: Serialize + Send + 'static>(
        workspace: impl Into<String>,
        data: T,
        compress: bool,
    ) -> Result<Vec<u8>> {
        let ws_id = workspace.into();
        spawn_blocking(move || -> Result<Vec<u8>> {
            CIPHER.get(ws_id)?.encrypt_binary::<T>(data, compress)
        })
            .await
            .map_err(map_log_err)?
    }

    pub async fn decrypt_json<T: for<'de> Deserialize<'de> + Send + 'static>(
        workspace: impl Into<String>,
        encrypted_data: String,
    ) -> Result<T> {
        let ws_id = workspace.into();
        spawn_blocking(move || -> Result<T> {
            CIPHER.get(ws_id)?.decrypt_json(encrypted_data)
        })
            .await
            .map_err(map_log_err)?
    }

    pub async fn decrypt_binary<T: for<'de> Deserialize<'de> + Send + 'static>(
        workspace: impl Into<String>,
        encrypted_data: Vec<u8>,
        compressed: bool,
    ) -> Result<T> {
        let ws_id = workspace.into();
        spawn_blocking(move || -> Result<T> {
            CIPHER.get(ws_id)?.decrypt_binary(&encrypted_data, compressed)
        })
            .await
            .map_err(map_log_err)?
    }

    pub fn base_path() -> Option<Arc<PathBuf>> {
        PATH.get().map(|p| p.clone())
    }

    pub fn get_path(workspace: impl AsRef<str>, id: impl AsRef<str>) -> Result<PathBuf> {
        let ws_id = workspace.as_ref();
        let id = id.as_ref();

        if let Some(path) = PATH.get().map(|p| p.join(ws_id).join(format!("{id}.bin"))) {
            Ok(path)
        } else {
            Err((StatusCode::NOT_FOUND, "path-not-found"))?
        }
    }
}

async fn put_cached<T: Send + Sync + 'static>(id: impl Into<String>, value: T) {
    let value_arc = Arc::new(Arc::new(RwLock::new(value)));
    let erased: Arc<dyn Any + Send + Sync> = value_arc;
    CACHE.insert(id.into(), erased).await;
}

async fn get_cached<T: Send + Sync + 'static>(id: impl AsRef<str>) -> Option<Arc<RwLock<T>>> {
    let erased = CACHE.get(id.as_ref()).await?;
    erased.downcast_ref::<Arc<RwLock<T>>>().cloned()
}

async fn pop_cached(id: impl AsRef<str>) {
    CACHE.invalidate(id.as_ref()).await;
}

async fn save_atomic<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<()> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.map_err(map_log_err)?;
    }

    let tmp = path.with_extension("tmp");

    {
        let mut f = fs::File::create(&tmp).await.map_err(map_log_err)?;
        f.write_all(data).await.map_err(map_log_err)?;
        f.sync_all().await.map_err(map_log_err)?;
    }

    let rename_res = fs::rename(&tmp, path).await;
    if let Err(_e) = rename_res {
        if fs::try_exists(path).await.unwrap_or(false) {
            let _ = fs::remove_file(path).await;
        }
        fs::rename(&tmp, path).await.map_err(map_log_err)?;
    }

    if let Some(parent) = path.parent() {
        if let Ok(dir) = fs::File::open(parent).await {
            let _ = dir.sync_all().await;
        }
    }

    Ok(())
}
