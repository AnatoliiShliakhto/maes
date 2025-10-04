use ::axum::Json;
use ::tokio::task::spawn_blocking;
use ::shared::{common::*, models::*, payloads::*, services::*};
use crate::{middleware::*, services::*};

pub async fn add_workspace_user(
    session: Session,
    Json(payload): Json<CreateWorkspaceUserPayload>,
) -> Result<()> {
    session.checked_admin()?;
    let ws_arc = WorkspaceService::get(&session.workspace).await?;

    let workspace = {
        let mut ws = ws_arc.write().await;
        let CreateWorkspaceUserPayload {
            username,
            login,
            password,
            unit,
            role,
        } = payload;
        ws.users
            .values()
            .any(|u| u.login == login)
            .then(|| Err((StatusCode::CONFLICT, "user-already-exists"))?);
        let hashed = spawn_blocking(move || hash_password(password))
            .await
            .map_err(map_server_err)??;
        let user = WorkspaceUser {
            id: safe_nanoid!(),
            username,
            login,
            password: hashed,
            unit,
            role,
        };

        ws.users.insert(user.id.clone(), user);
        ws.metadata.update(&session.username);

        ws.clone()
    };

    WorkspaceService::upsert(workspace).await
}

pub async fn delete_workspace_user(
    session: Session,
    Json(payload): Json<DeleteWorkspaceUserPayload>,
) -> Result<()> {
    session.checked_admin()?;
    if payload.id == session.id { Err((StatusCode::CONFLICT, "cannot-delete-self"))? }
    let ws_arc = WorkspaceService::get(&session.workspace).await?;
}