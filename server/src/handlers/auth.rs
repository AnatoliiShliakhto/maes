use crate::{middleware::*, service::*};
use ::axum::Json;
use ::shared::{common::*, models::*, payloads::*, services::*, utils::*};

pub async fn authorize(
    connection: Connection,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Claims>> {
    connection.checked()?;

    let (user, workspace, workspace_name, key, path) = {
        let ws = WorkspaceService::get(&payload.workspace).await?;
        let ws = ws.read().await;

        let user = match ws.users.values().find(|u| u.login == payload.login) {
            Some(u) => u.clone(),
            None => return Err((StatusCode::NOT_FOUND, "credentials-not-found").into()),
        };

        verify_password(&payload.password, &user.password)?;

        let path = ws.unit_tree.node_path(&user.unit);
        let ws_id = ws.id.clone();
        let ws_name = ws.name.clone();
        let ws_key = ws.key.clone();

        (user, ws_id, ws_name,ws_key, path)
    };

    let session = ClientSession {
        id: safe_nanoid!(),
        workspace,
        workspace_name,
        key,
        username: user.username,
        unit: user.unit,
        path,
        role: user.role,
    };

    let claims = SessionService::add_session(session).await;
    Ok(Json(claims))
}

pub async fn logout(session: Session) -> Result<()> {
    SessionService::remove_session(&session.token).await;
    Ok(())
}
