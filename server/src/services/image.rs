use crate::common::*;
use ::futures::{StreamExt, stream};
use ::image::{DynamicImage, ImageFormat, imageops::FilterType};
use ::shared::common::*;
use ::std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    sync::Arc,
};
use ::tokio::{fs, task};
use ::tracing::error;

const MAX_DIM: u32 = 300;

#[derive(Copy, Clone)]
pub struct ImageService;

impl ImageService {
    pub async fn convert_and_save(
        input_path: impl AsRef<Path>,
        workspace: impl AsRef<str>,
        entity: impl AsRef<str>,
        id: impl AsRef<str>,
    ) -> Result<()> {
        let input_path = input_path.as_ref().to_owned();
        let dir = State::path().join(format!(
            "assets/{workspace}/{entity}",
            workspace = workspace.as_ref(),
            entity = entity.as_ref()
        ));
        fs::create_dir_all(&dir).await.map_err(map_log_err)?;
        let output_path = dir.join(format!("{id}.webp", id = id.as_ref()));

        task::spawn_blocking(move || {
            Self::resize_and_convert_to_webp(&input_path, &output_path, MAX_DIM)
        })
        .await
        .map_err(map_log_err)?
    }

    pub async fn remove(
        workspace: impl AsRef<str>,
        entity: impl AsRef<str>,
        id: impl AsRef<str>,
    ) -> Result<()> {
        let path = State::path().join(format!(
            "assets/{workspace}/{entity}/{id}.webp",
            workspace = workspace.as_ref(),
            entity = entity.as_ref(),
            id = id.as_ref()
        ));

        match fs::remove_file(path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(map_log_err(e)),
        }
    }

    pub async fn remove_entities(workspace: impl AsRef<Path>, entities: Vec<String>) -> Result<()> {
        let base = State::path().join("assets").join(workspace.as_ref());
        tokio::spawn(async move {
            for entity in entities {
                let dir = base.join(entity);
                fs::remove_dir_all(dir).await.ok();
            }
        });
        Ok(())
    }

    pub async fn remove_workspace(workspace: impl AsRef<str>) -> Result<()> {
        let dir = State::path().join("assets").join(workspace.as_ref());
        fs::remove_dir_all(dir).await.map_err(map_log_err)
    }

    pub async fn batch_remove(
        workspace: impl Into<String>,
        entity: impl Into<String>,
        ids: HashSet<String>,
    ) -> Result<()> {
        let ws_id = workspace.into();
        let entity_id = entity.into();

        tokio::spawn(async move {
            let sem = Arc::new(tokio::sync::Semaphore::new(8));
            for id in ids {
                let ws = ws_id.clone();
                let en = entity_id.clone();
                let sem = sem.clone();
                tokio::spawn(async move {
                    let _permit = sem.acquire().await;
                    let _ = Self::remove(&ws, &en, id).await;
                });
            }
        });
        Ok(())
    }

    pub fn copy_images(
        source_workspace: impl AsRef<Path>,
        source_entity: impl AsRef<Path>,
        destination_workspace: impl AsRef<Path>,
        destination_entity: impl AsRef<Path>,
    ) -> Result<()> {
        let src = State::path()
            .join("assets")
            .join(source_workspace)
            .join(source_entity);
        let dst = State::path()
            .join("assets")
            .join(destination_workspace)
            .join(destination_entity);

        task::spawn(async move {
            if let Err(e) = fs::create_dir_all(&dst).await {
                error!("create_dir_all error: {e}");
                return;
            }

            let mut entries = match fs::read_dir(&src).await {
                Ok(rd) => rd,
                Err(e) => {
                    error!("read_dir error: {e}");
                    return;
                }
            };

            let mut paths = Vec::new();
            while let Ok(Some(ent)) = entries.next_entry().await {
                let ft = match ent.file_type().await {
                    Ok(ft) => ft,
                    Err(e) => {
                        eprintln!("file_type error: {e}");
                        continue;
                    }
                };
                if !ft.is_file() {
                    continue;
                }
                paths.push(ent.path());
            }

            let sem = Arc::new(tokio::sync::Semaphore::new(8));
            stream::iter(paths.into_iter())
                .for_each_concurrent(8, |src_path| {
                    let sem = sem.clone();
                    let dst_path = dst.join(src_path.file_name().unwrap_or_default());
                    async move {
                        let _permit = sem.acquire().await;
                        if fs::try_exists(&dst_path).await.unwrap_or(false) {
                            return;
                        }
                        if let Some(parent) = dst_path.parent() {
                            if let Err(e) = fs::create_dir_all(parent).await {
                                error!("create parent error: {e}");
                                return;
                            }
                        }
                        if let Err(e) = fs::copy(&src_path, &dst_path).await {
                            if e.kind() != std::io::ErrorKind::AlreadyExists {
                                error!(
                                    "copy error {} -> {}: {e}",
                                    src_path.display(),
                                    dst_path.display()
                                );
                            }
                        }
                    }
                })
                .await;
        });
        Ok(())
    }

    fn resize_and_convert_to_webp(
        input_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
        max_dim: u32,
    ) -> Result<()> {
        let img: DynamicImage = image::open(&input_path).map_err(map_log_err)?;

        let resized = if img.width() > max_dim || img.height() > max_dim {
            img.resize(max_dim, max_dim, FilterType::Lanczos3)
        } else {
            img
        };

        if let Some(parent) = output_path.as_ref().parent() {
            std::fs::create_dir_all(parent).map_err(map_log_err)?;
        }

        let file = File::create(output_path).map_err(map_log_err)?;
        let mut writer = BufWriter::new(file);
        resized
            .write_to(&mut writer, ImageFormat::WebP)
            .map_err(map_log_err)?;

        writer.flush().map_err(map_log_err)?;
        Ok(())
    }

    pub async fn get_entity_images(
        workspace: impl AsRef<str>,
        entity: impl AsRef<str>,
    ) -> Result<HashSet<String>> {
        let dir = State::path().join(format!(
            "assets/{workspace}/{entity}",
            workspace = workspace.as_ref(),
            entity = entity.as_ref()
        ));

        match fs::read_dir(&dir).await {
            Ok(mut rd) => {
                let mut names = HashSet::with_capacity(128);
                while let Some(entry) = rd.next_entry().await.map_err(map_log_err)? {
                    let ft = entry.file_type().await.map_err(map_log_err)?;
                    if !ft.is_file() {
                        continue;
                    }
                    if let Ok(filename) = entry.file_name().into_string() {
                        if filename.ends_with(".webp") {
                            let trimmed = &filename[..filename.len() - 5];
                            names.insert(trimmed.to_string());
                        }
                    }
                }
                Ok(names)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(HashSet::new()),
            Err(e) => Err(map_log_err(e)),
        }
    }
}
