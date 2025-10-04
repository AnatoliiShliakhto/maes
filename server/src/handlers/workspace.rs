use crate::{middleware::*, repositories::*, services::*};
use ::axum::{Json, extract::Path};
use ::futures::{StreamExt, TryStreamExt, stream};
use ::indexmap::IndexMap;
use ::shared::{common::*, models::*, payloads::*, services::*, utils::*};
use ::std::{collections::HashSet, str::FromStr};
use ::tokio::{fs, task::spawn_blocking};

pub async fn list_workspaces(connection: Connection) -> Result<Json<Vec<WorkspaceMetadata>>> {
    connection.checked()?;
    let workspaces = load_workspaces_meta().await?;
    Ok(Json(workspaces))
}

pub async fn create_workspace(
    connection: Connection,
    Json(payload): Json<CreateWorkspacePayload>,
) -> Result<Json<Workspace>> {
    connection.checked()?;
    let CreateWorkspacePayload {
        name,
        username,
        login,
        password,
    } = payload;

    let raw_password = password;
    let hashed = spawn_blocking(move || hash_password(raw_password))
        .await
        .map_err(map_log_err)??;

    let user = WorkspaceUser {
        id: safe_nanoid!(),
        username,
        login,
        password: hashed,
        node: "".to_string(),
        path: "".to_string(),
        role: WorkspaceRole::Admin,
    };
    let mut workspace = Workspace {
        id: safe_nanoid!(),
        name,
        metadata: Metadata::new(&user.username),
        ..Default::default()
    };
    workspace.users.insert(user.id.clone(), user);

    let entities = Entities::new(workspace.to_entity());
    Store::upsert(entities).await?;

    let students = Students::new(&workspace.id);
    Store::upsert(students).await?;

    init_workspace_meta(&workspace).await?;
    TaskRepository::init(&workspace.id).await?;
    Store::upsert::<Workspace>(workspace.clone()).await?;
    Ok(Json(workspace))
}

pub async fn delete_workspace(
    connection: Connection,
    Path(id): Path<String>,
) -> Result<Json<String>> {
    connection.checked()?;
    Store::remove_workspace(&id).await?;
    Ok(Json(id))
}

pub async fn get_workspace_tree(
    session: Session,
    Path(kind): Path<String>,
) -> Result<Json<Vec<TreeNode>>> {
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;

    let mut tree_map = {
        let ws_guard = ws_arc.read().await;
        get_ref_tree(&ws_guard, kind)?.clone()
    };

    tree_map.populate_children();

    Ok(Json(tree_map.into_values().collect()))
}

pub async fn create_workspace_treenode(
    session: Session,
    Path(kind): Path<String>,
    Json(payload): Json<CreateWorkspaceTreeNodePayload>,
) -> Result<Json<TreeNode>> {
    session.checked_admin()?;
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;
    let CreateWorkspaceTreeNodePayload { name, parent } = payload;

    let (snapshot, node) = {
        let mut ws_guard = ws_arc.write().await;

        let node = TreeNode {
            id: safe_nanoid!(),
            parent,
            name,
            children: vec![],
        };
        let key = node.id.clone();
        let tree_map = get_mut_tree(&mut ws_guard, kind)?;

        tree_map.insert(key, node.clone());
        tree_map.sort_by_name();
        ws_guard.metadata.update(&session.username);

        (ws_guard.clone(), node)
    };

    EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
    Store::upsert(snapshot).await?;
    Ok(Json(node))
}

pub async fn update_workspace_treenode(
    session: Session,
    Path((kind, node_id)): Path<(String, String)>,
    Json(payload): Json<UpdateWorkspaceTreeNodePayload>,
) -> Result<Json<TreeNode>> {
    session.checked_admin()?;
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;
    let UpdateWorkspaceTreeNodePayload { name, parent } = payload;

    let (snapshot, node) = {
        let mut ws_guard = ws_arc.write().await;
        let tree_map = get_mut_tree(&mut ws_guard, kind)?;

        let node = tree_map
            .get_mut(&node_id)
            .ok_or((StatusCode::NOT_FOUND, "entity-not-found"))?;
        node.name = name;
        node.parent = parent;
        let node = node.clone();

        tree_map.sort_by_name();
        ws_guard.metadata.update(&session.username);

        (ws_guard.clone(), node)
    };

    EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
    Store::upsert(snapshot).await?;
    Ok(Json(node))
}

pub async fn delete_workspace_treenode(
    session: Session,
    Path((kind, node_id)): Path<(String, String)>,
) -> Result<Json<String>> {
    session.checked_admin()?;
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;

    let (snapshot, nodes) = {
        let mut ws_guard = ws_arc.write().await;
        let tree_map = get_mut_tree(&mut ws_guard, kind)?;
        let nodes = tree_map
            .node_descendants(&node_id)
            .into_iter()
            .collect::<HashSet<String>>();
        tree_map.remove_node(&node_id);

        if kind == EntityKind::Workspace {
            ws_guard.users.retain(|_, u| !nodes.contains(&u.node));
        }

        ws_guard.metadata.update(&session.username);

        (ws_guard.clone(), nodes)
    };

    if kind != EntityKind::Workspace {
        let vec = nodes.into_iter().collect::<Vec<_>>();
        EntityRepository::batch_remove(&session.workspace, None, Some(vec.clone())).await?;
    }

    Store::upsert(snapshot).await?;
    Ok(Json(node_id))
}

fn get_ref_tree(ws: &Workspace, entity_kind: EntityKind) -> Result<&IndexMap<String, TreeNode>> {
    Ok(match entity_kind {
        EntityKind::Workspace => &ws.unit_tree,
        EntityKind::Quiz => &ws.quiz_tree,
        EntityKind::Survey => &ws.survey_tree,
        EntityKind::Checklist => &ws.checklist_tree,
        _ => Err((StatusCode::NOT_FOUND, "entity-not-found"))?,
    })
}

fn get_mut_tree(
    ws: &mut Workspace,
    entity_kind: EntityKind,
) -> Result<&mut IndexMap<String, TreeNode>> {
    Ok(match entity_kind {
        EntityKind::Workspace => &mut ws.unit_tree,
        EntityKind::Quiz => &mut ws.quiz_tree,
        EntityKind::Survey => &mut ws.survey_tree,
        EntityKind::Checklist => &mut ws.checklist_tree,
        _ => Err((StatusCode::NOT_FOUND, "entity-not-found"))?,
    })
}

async fn init_workspace_meta(workspace: &Workspace) -> Result<()> {
    let path = Store::get_path(&workspace.id, WORKSPACE)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.map_err(map_log_err)?;
    }
    let metadata = WorkspaceMetadata {
        id: workspace.id.clone(),
        name: workspace.name.clone(),
    };
    let encrypted = Store::encrypt_binary(&workspace.id, metadata, false).await?;
    fs::write(&path, encrypted).await.map_err(map_log_err)
}
async fn load_workspaces_meta() -> Result<Vec<WorkspaceMetadata>> {
    let Some(base) = Store::base_path() else {
        return Ok(vec![]);
    };

    let mut rd = fs::read_dir(&*base).await.map_err(map_log_err)?;
    let mut names = Vec::with_capacity(128);
    while let Some(entry) = rd.next_entry().await.map_err(map_log_err)? {
        if entry.file_type().await.map_err(map_log_err)?.is_dir() {
            if let Ok(ws) = entry.file_name().into_string() {
                names.push(ws);
            }
        }
    }

    let cpu = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let concurrency = cpu.saturating_mul(4).min(32);

    let metas = stream::iter(names)
        .map(|workspace| async move {
            let path = Store::get_path(&workspace, WORKSPACE)?;
            match fs::read(&path).await {
                Ok(encrypted) => {
                    let meta =
                        Store::decrypt_binary::<WorkspaceMetadata>(workspace, encrypted, false)
                            .await?;
                    Ok::<Option<WorkspaceMetadata>, Error>(Some(meta))
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(map_log_err(e)),
            }
        })
        .buffer_unordered(concurrency)
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .flatten()
        .collect();

    Ok(metas)
}
