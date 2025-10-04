use crate::{middleware::*, service::*};
use ::axum::{Json, extract::Path};
use ::indexmap::IndexMap;
use ::shared::{common::*, models::*, payloads::*, services::*, utils::*};
use ::tokio::task::spawn_blocking;

pub async fn list_workspaces(connection: Connection) -> Result<Json<Vec<Entity>>> {
    connection.checked()?;
    let workspaces = WorkspaceService::list_workspaces().await?;
    Ok(Json(workspaces))
}

pub async fn get_workspace(session: Session) -> Result<Json<Workspace>> {
    let ws_arc = WorkspaceService::get(&session.workspace).await?;
    let ws = ws_arc.read().await.clone();
    Ok(Json(ws))
}

pub async fn create_workspace(
    connection: Connection,
    Json(payload): Json<CreateWorkspacePayload>,
) -> Result<()> {
    connection.checked()?;

    let raw_password = payload.password;
    let hashed = spawn_blocking(move || hash_password(raw_password))
        .await
        .map_err(map_server_err)??;

    let user = WorkspaceUser {
        id: safe_nanoid!(),
        username: payload.username,
        login: payload.login,
        password: hashed,
        unit: "".to_string(),
        role: WorkspaceRole::Admin,
    };
    let mut workspace = Workspace {
        id: safe_nanoid!(),
        key: safe_nanoid!(32),
        name: payload.name,
        metadata: Metadata::new(&user.username),
        ..Default::default()
    };
    workspace.users.insert(user.id.clone(), user);
    WorkspaceService::upsert(workspace).await
}

pub async fn get_workspace_tree(
    session: Session,
    Path(kind): Path<EntityKind>,
) -> Result<Json<Vec<TreeNode>>> {
    let ws_arc = WorkspaceService::get(&session.workspace).await?;

    let mut tree_map = {
        let ws = ws_arc.read().await;
        get_ref_tree(&ws, kind)?.clone()
    };

    tree_map.populate_children();

    Ok(Json(tree_map.into_values().collect()))
}

pub async fn create_workspace_treenode(
    session: Session,
    Path(kind): Path<EntityKind>,
    Json(payload): Json<CreateWorkspaceTreeNodePayload>,
) -> Result<()> {
    session.checked_admin()?;
    let ws_arc = WorkspaceService::get(&session.workspace).await?;

    let workspace = {
        let mut ws = ws_arc.write().await;

        let CreateWorkspaceTreeNodePayload { id, name, parent } = payload;
        let node = TreeNode {
            id,
            parent,
            name,
            children: vec![],
        };
        let key = node.id.clone();
        let tree_map = get_mut_tree(&mut ws, kind)?;

        tree_map.insert(key, node);
        ws.metadata.update(&session.username);

        ws.clone()
    };

    WorkspaceService::upsert(workspace).await
}

pub async fn update_workspace_treenode(
    session: Session,
    Path(kind): Path<EntityKind>,
    Json(payload): Json<UpdateWorkspaceTreeNodePayload>,
) -> Result<()> {
    session.checked_admin()?;
    let ws_arc = WorkspaceService::get(&session.workspace).await?;

    let workspace = {
        let mut ws = ws_arc.write().await;
        let UpdateWorkspaceTreeNodePayload { id, name } = payload;
        let tree_map = get_mut_tree(&mut ws, kind)?;

        tree_map
            .get_mut(&id)
            .ok_or((StatusCode::NOT_FOUND, "entity-not-found"))?
            .name = name;
        ws.metadata.update(&session.username);

        ws.clone()
    };

    WorkspaceService::upsert(workspace).await
}

pub async fn delete_workspace_treenode(
    session: Session,
    Path(kind): Path<EntityKind>,
    Json(payload): Json<DeleteWorkspaceTreeNodePayload>,
) -> Result<()> {
    session.checked_admin()?;
    let ws_arc = WorkspaceService::get(&session.workspace).await?;

    let workspace = {
        let mut ws = ws_arc.write().await;
        let DeleteWorkspaceTreeNodePayload { id } = payload;
        let tree_map = get_mut_tree(&mut ws, kind)?;
        tree_map.shift_remove(&id);
        ws.metadata.update(&session.username);

        ws.clone()
    };

    WorkspaceService::upsert(workspace).await
}

fn get_ref_tree(ws: &Workspace, entity_kind: EntityKind) -> Result<&IndexMap<String, TreeNode>> {
    Ok(match entity_kind {
        EntityKind::Workspace => &ws.unit_tree,
        EntityKind::Quizz => &ws.quiz_tree,
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
        EntityKind::Quizz => &mut ws.quiz_tree,
        EntityKind::Survey => &mut ws.survey_tree,
        EntityKind::Checklist => &mut ws.checklist_tree,
        _ => Err((StatusCode::NOT_FOUND, "entity-not-found"))?,
    })
}
