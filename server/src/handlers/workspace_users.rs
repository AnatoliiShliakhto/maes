use crate::{middleware::*, services::*};
use ::axum::{Json, extract::Path};
use ::shared::{common::*, models::*, payloads::*, services::*, utils::*};
use ::std::collections::HashSet;
use ::tokio::task::spawn_blocking;


pub async fn list_workspace_users(session: Session) -> Result<Json<Vec<WorkspaceUser>>> {
    let node_id = session.node.clone();
    list_workspace_users_by_node(session, Path(node_id)).await
}

pub async fn list_workspace_users_by_node(
    session: Session,
    Path(node_id): Path<String>,
) -> Result<Json<Vec<WorkspaceUser>>> {
    session.checked_supervisor()?;
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;

    let mut users = {
        let ws_guard = ws_arc.read().await;
        let node_ids_vec = ws_guard.unit_tree.node_descendants(node_id);
        let node_ids = node_ids_vec
            .iter()
            .map(|s| s.as_str())
            .collect::<HashSet<&str>>();

        ws_guard.users
            .values()
            .filter(|u| node_ids.contains(u.node.as_str()))
            .map(|u| WorkspaceUser {
                id: u.id.clone(),
                username: u.username.clone(),
                login: String::new(),
                password: String::new(),
                node: u.node.clone(),
                path: u.path.clone(),
                role: u.role.clone(),
            })
            .collect::<Vec<_>>()
    };

    users.sort_unstable_by(|a, b| {
        a.username.cmp(&b.username)
        // a.username
        //     .as_bytes()
        //     .iter()
        //     .map(|c| c.to_ascii_lowercase())
        //     .cmp(b.username.as_bytes().iter().map(|c| c.to_ascii_lowercase()))
    });

    Ok(Json(users))
}

pub async fn add_workspace_user(
    session: Session,
    Json(payload): Json<CreateWorkspaceUserPayload>,
) -> Result<Json<WorkspaceUser>> {
    session.checked_admin()?;
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;
    let CreateWorkspaceUserPayload {
        username,
        login,
        password,
        node,
        role,
    } = payload;

    let hashed = spawn_blocking(move || hash_password(password))
        .await
        .map_err(map_log_err)??;

    let (snapshot, user) = {
        let mut ws_guard = ws_arc.write().await;

        if ws_guard.users.values().any(|u| u.login == login) {
            Err((StatusCode::CONFLICT, "user-already-exists"))?
        }

        let path = ws_guard.unit_tree.node_path(&node);
        let user = WorkspaceUser {
            id: safe_nanoid!(),
            username,
            login,
            password: hashed,
            node: if role == WorkspaceRole::Supervisor || role == WorkspaceRole::Admin { "".to_string() } else { node },
            path: if role == WorkspaceRole::Supervisor || role == WorkspaceRole::Admin { "".to_string() } else { path },
            role,
        };

        ws_guard.users.insert(user.id.clone(), user.clone());
        ws_guard.metadata.update(&session.username);

        (ws_guard.clone(), user)
    };

    Store::upsert(snapshot).await?;
    Ok(Json(user))
}

pub async fn delete_workspace_user(
    session: Session,
    Path(user_id): Path<String>,
) -> Result<Json<String>> {
    session.checked_admin()?;
    if session.id == user_id {
        Err((StatusCode::CONFLICT, "cannot-delete-self"))?
    }

    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;
    let snapshot = {
        let mut ws_guard = ws_arc.write().await;
        ws_guard.users.shift_remove(&user_id);
        ws_guard.metadata.update(&session.username);

        ws_guard.clone()
    };

    Store::upsert(snapshot).await?;
    Ok(Json(user_id))
}
