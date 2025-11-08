use crate::{common::*, repositories::*, services::*};
use ::shared::{common::*, models::*};
use ::std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Arc,
};
use ::tokio::sync::RwLock;
use ::zip::{
    CompressionMethod, ZipArchive, ZipWriter,
    write::{ExtendedFileOptions, FileOptions},
};

const EXPORT_WORKSPACE: [EntityKind; 3] =
    [EntityKind::Workspace, EntityKind::Quiz, EntityKind::Survey];

const EXPORT_ENTITIES: [EntityKind; 3] =
    [EntityKind::QuizRecord, EntityKind::SurveyRecord, EntityKind::Json];

pub struct ExchangeService;

impl ExchangeService {
    pub fn init() {
        let path = State::path();
        let temp_dir = path.join("temp");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).ok();
        }
    }

    pub async fn export_workspace(
        workspace: impl Into<String>,
        dest_zip: impl AsRef<Path>,
    ) -> Result<()> {
        let ws_id = workspace.into();
        let path = State::path();
        let temp_path = Self::mk_temp_dir(&path)?;

        let entities =
            EntityRepository::list_by_filter(&ws_id, Some(EXPORT_WORKSPACE.to_vec()), None, None).await?;
        let ws = entities
            .iter()
            .find(|e| e.kind == EntityKind::Workspace)
            .ok_or((StatusCode::NOT_FOUND, "workspace-not-found"))?;

        let mut payload =
            build_entities_payload(&path, &ws_id, entities.iter().map(|e| e.id.clone()));
        payload.push((
            path.join(format!("assets/{ws_id}")),
            "assets".to_string(),
        ));

        let ws_meta = WorkspaceMetadata {
            id: ws.id.clone(),
            name: ws.name.clone(),
            version: ws.metadata.updated_at,
        };

        let ws_meta_str = serde_json::to_string_pretty(&ws_meta).map_err(map_log_err)?;
        let ws_meta_path = temp_path.join("workspace.json");
        fs::write(&ws_meta_path, ws_meta_str).map_err(map_log_err)?;
        payload.push((ws_meta_path, "workspace.json".to_string()));

        let encrypted = Store::encrypt_binary("", entities, false).await?;
        let entities_bin_path = temp_path.join("entities.bin");
        fs::write(&entities_bin_path, encrypted).map_err(map_log_err)?;
        payload.push((entities_bin_path, "entities.bin".to_string()));

        zip_many(&payload, dest_zip).map_err(map_log_err)?;
        fs::remove_dir_all(temp_path).map_err(map_log_err)?;
        Ok(())
    }

    pub async fn export(
        workspace: impl Into<String>,
        entities: Vec<String>,
        dest_zip: impl AsRef<Path>,
    ) -> Result<()> {
        let ws_id = workspace.into();
        let path = State::path();
        let temp_path = Self::mk_temp_dir(&path)?;

        let entities = EntityRepository::list_by_filter(
            &ws_id,
            Some(EXPORT_ENTITIES.to_vec()),
            Some(entities),
            None
        )
        .await?;

        let mut payload =
            build_entities_payload(&path, &ws_id, entities.iter().map(|e| e.id.clone()));

        let ws_meta = {
            let ws_arc = Store::find::<Workspace>(&ws_id, &ws_id).await?;
            let ws_guard = ws_arc.read().await;
            WorkspaceMetadata {
                id: ws_guard.id.clone(),
                name: ws_guard.name.clone(),
                version: ws_guard.metadata.updated_at,
            }
        };

        let ws_meta_str = serde_json::to_string_pretty(&ws_meta).map_err(map_log_err)?;
        let ws_meta_path = temp_path.join("workspace.json");
        fs::write(&ws_meta_path, ws_meta_str).map_err(map_log_err)?;
        payload.push((ws_meta_path, "workspace.json".to_string()));

        let encrypted = Store::encrypt_binary("", entities, false).await?;
        let entities_bin_path = temp_path.join("entities.bin");
        fs::write(&entities_bin_path, encrypted).map_err(map_log_err)?;
        payload.push((entities_bin_path, "entities.bin".to_string()));

        zip_many(&payload, dest_zip).map_err(map_log_err)?;
        fs::remove_dir_all(temp_path).map_err(map_log_err)?;
        Ok(())
    }

    pub async fn import(src_zip: impl AsRef<Path>) -> Result<()> {
        let path = State::path();
        let temp_path = Self::mk_temp_dir(&path)?;
        unzip_to_dir(src_zip, &temp_path).map_err(map_log_err)?;

        let meta: WorkspaceMetadata = {
            let text = fs::read_to_string(temp_path.join("workspace.json")).map_err(map_log_err)?;
            serde_json::from_str(&text).map_err(map_log_err)?
        };
        let import_entities_vec = fs::read(temp_path.join("entities.bin")).map_err(map_log_err)?;
        let import_entities =
            Store::decrypt_binary::<Vec<Entity>>("", import_entities_vec, false).await?;

        if let Some(ws) = import_entities
            .iter()
            .find(|e| e.kind == EntityKind::Workspace)
        {
            Self::import_workspace(ws).await?;
        }

        let Ok(entities_arc) = Store::find::<Entities>(&meta.id, ENTITIES).await else {
            return Err((StatusCode::NOT_FOUND, "workspace-not-found"))?;
        };
        let (snapshot, updated_ids) =
            upsert_imported_entities(entities_arc, &import_entities).await;
        Store::upsert(snapshot).await?;

        apply_import_fs_changes(&path, &temp_path, &meta.id, updated_ids);

        fs::remove_dir_all(temp_path).map_err(map_log_err)?;
        Ok(())
    }

    fn mk_temp_dir(root: &Path) -> Result<PathBuf> {
        let temp_path = root.join(format!("temp/{}", safe_nanoid!()));
        fs::create_dir_all(&temp_path).map_err(map_log_err)?;
        Ok(temp_path)
    }

    async fn import_workspace(entity: &Entity) -> Result<()> {
        if let Ok(ws_arc) = Store::find::<Workspace>(&entity.id, &entity.id).await {
            if ws_arc.read().await.metadata.updated_at >= entity.metadata.updated_at {
                return Err((StatusCode::CONFLICT, "workspace-version-conflict"))?;
            }
        } else {
            let entities = Entities::new_with_id(&entity.id);
            Store::upsert(entities).await?;

            let students = Students::new(&entity.id);
            Store::upsert(students).await?;

            TaskRepository::init(&entity.id).await?;

            let metadata = WorkspaceMetadata {
                id: entity.id.clone(),
                name: entity.name.clone(),
                version: entity.metadata.updated_at,
            };
            let encrypted = Store::encrypt_binary(&entity.id, metadata, false).await?;
            let path = State::path().join(format!("workspaces/{id}/workspace.bin", id = entity.id));
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).map_err(map_log_err)?;
            }
            tokio::fs::write(&path, encrypted)
                .await
                .map_err(map_log_err)?;
        }
        Ok(())
    }
}

pub fn move_dir_replace(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if !src.exists() {
        return Ok(());
    }
    if dst.exists() {
        fs::remove_dir_all(dst)?;
    }
    match fs::rename(src, dst) {
        Ok(_) => Ok(()),
        Err(_) => {
            copy_dir_recursive(src, dst)?;
            fs::remove_dir_all(src)
        }
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else if ty.is_file() {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&from, &to)?;
        } else {
            // skip symlinks/others
        }
    }
    Ok(())
}

pub fn copy_file(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<u64> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(src, dst)
}

pub fn move_file(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)?;
    }
    match fs::rename(src, dst) {
        Ok(_) => Ok(()),
        Err(_) => {
            fs::copy(src, dst)?;
            fs::remove_file(src)
        }
    }
}

fn build_entities_payload<P: AsRef<Path>, I: IntoIterator<Item = String>>(
    root: P,
    ws_id: &str,
    ids: I,
) -> Vec<(PathBuf, String)> {
    let root = root.as_ref();
    ids.into_iter()
        .map(|id| {
            (
                root.join(format!("workspaces/{ws_id}/{id}.bin")),
                format!("entities/{id}.bin"),
            )
        })
        .collect()
}

fn apply_import_fs_changes(root: &Path, temp: &Path, ws_id: &str, updated_ids: Vec<String>) {
    for id in updated_ids {
        let src_path = temp.join(format!("entities/{id}.bin"));
        if src_path.exists() {
            let dest_path = root.join(format!("workspaces/{ws_id}/{id}.bin"));
            _ = move_file(src_path, dest_path);
        }

        let src_dir = temp.join(format!("assets/{id}"));
        let dest_dir = root.join(format!("assets/{ws_id}/{id}"));
        _ = move_dir_replace(src_dir, dest_dir);
    }
}

async fn upsert_imported_entities(
    entities_arc: Arc<RwLock<Entities>>,
    import_entities: &Vec<Entity>,
) -> (Entities, Vec<String>) {
    let mut updated = vec![];
    let snapshot = {
        let mut guard = entities_arc.write().await;
        for entity in import_entities.iter() {
            if let Some(e) = guard.get_mut(&entity.id) {
                if entity.metadata.updated_at <= e.metadata.updated_at {
                    continue;
                }
                e.metadata = entity.metadata.clone();
                e.kind = entity.kind.clone();
                e.name = entity.name.clone();
            } else {
                guard.insert(entity.id.clone(), entity.clone());
            }
            updated.push(entity.id.clone());
        }
        guard.clone()
    };
    (snapshot, updated)
}

fn zip_many<S, D, Z>(pairs: &[(S, D)], zip_path: Z) -> io::Result<()>
where
    S: AsRef<Path>,
    D: AsRef<str>,
    Z: AsRef<Path>,
{
    let file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(file);
    let opts: FileOptions<'static, ExtendedFileOptions> =
        FileOptions::default().compression_method(CompressionMethod::Deflated);

    for (src_ref, dest_ref) in pairs {
        let src = src_ref.as_ref();
        let mut dest_root = normalize_zip_path(dest_ref.as_ref());

        if src.is_file() {
            let file_name = src
                .file_name()
                .map(|s| s.to_string_lossy())
                .unwrap_or_default();
            let inside = if dest_root.is_empty() || dest_root.ends_with('/') {
                format!("{dest_root}{file_name}")
            } else {
                dest_root.clone()
            };
            write_file_into_zip(&mut zip, src, &inside, opts.clone())?;
        } else if src.is_dir() {
            if !dest_root.is_empty() && !dest_root.ends_with('/') {
                dest_root.push('/');
            }
            for entry in walkdir::WalkDir::new(src)
                .into_iter()
                .filter_map(|res: walkdir::Result<walkdir::DirEntry>| res.ok())
            {
                let p = entry.path();
                let rel = match p.strip_prefix(src) {
                    Ok(r) => r,
                    Err(_) => continue,
                };
                if rel.as_os_str().is_empty() {
                    if entry.file_type().is_dir() {
                        let dir_name = trim_leading_slashes(&dest_root);
                        if !dir_name.is_empty() {
                            zip.add_directory(dir_name, opts.clone())?;
                        }
                    }
                    continue;
                }

                let inside = format!("{dest_root}{}", rel.to_string_lossy().replace('\\', "/"));
                if entry.file_type().is_dir() {
                    zip.add_directory(ensure_trailing_slash(&inside), opts.clone())?;
                } else if entry.file_type().is_file() {
                    write_file_into_zip(&mut zip, p, &inside, opts.clone())?;
                }
            }
        }
    }

    zip.finish()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}

pub fn unzip_to_dir(zip_path: impl AsRef<Path>, dest_dir: impl AsRef<Path>) -> io::Result<()> {
    let zip_path = zip_path.as_ref();
    let dest_dir = dest_dir.as_ref();
    fs::create_dir_all(dest_dir)?;

    let file = File::open(zip_path)?;
    let mut archive =
        ZipArchive::new(file).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let out_path = sanitize_extract_path(dest_dir, entry.name())?;

        if entry.name().ends_with('/') {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = File::create(&out_path)?;
            io::copy(&mut entry, &mut outfile)?;
        }
    }
    Ok(())
}

fn write_file_into_zip<W: Write + io::Seek>(
    zip: &mut ZipWriter<W>,
    src: &Path,
    inside_path: &str,
    opts: FileOptions<'_, ExtendedFileOptions>,
) -> io::Result<()> {
    let mut f = File::open(src)?;
    zip.start_file(normalize_zip_path(&inside_path), opts)
        .map_err(to_io)?;
    io::copy(&mut f, zip)?;
    Ok(())
}

fn normalize_zip_path(p: &str) -> String {
    let s = p.replace('\\', "/");
    let s = s.strip_prefix("./").unwrap_or(&s).to_string();
    let mut out = String::new();
    for comp in s.split('/') {
        match comp {
            "" | "." => continue,
            ".." => continue,
            c => {
                if !out.is_empty() {
                    out.push('/');
                }
                out.push_str(c);
            }
        }
    }
    out
}

fn ensure_trailing_slash(s: &str) -> String {
    if s.ends_with('/') {
        s.to_string()
    } else {
        format!("{s}/")
    }
}

fn trim_leading_slashes(s: &str) -> &str {
    let mut i = 0usize;
    let b = s.as_bytes();
    while i < b.len() && b[i] == b'/' {
        i += 1;
    }
    &s[i..]
}

fn sanitize_extract_path(base: &Path, name: &str) -> io::Result<PathBuf> {
    let unsafe_path = Path::new(name);
    let mut out = base.to_path_buf();
    for comp in unsafe_path.components() {
        use std::path::Component::*;
        match comp {
            Prefix(_) | RootDir => continue,
            CurDir => continue,
            ParentDir => (),
            Normal(p) => out.push(p),
        }
    }
    Ok(out)
}

fn to_io<E: std::fmt::Display>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{e}"))
}
