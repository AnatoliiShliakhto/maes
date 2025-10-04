use crate::{middleware::*, services::*};
use ::axum::Json;
use ::shared::{common::*, models::*, payloads::*, services::*, utils::*};

pub async fn authorize(
    connection: Connection,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<Claims>> {
    connection.checked()?;

    let (workspace, workspace_name, user, path) = {
        let AuthPayload {
            workspace,
            login,
            password,
        } = payload;
        let ws_arc = Store::find::<Workspace>(workspace.clone(), workspace).await?;
        let ws_guard = ws_arc.read().await;

        let user = match ws_guard.users.values().find(|u| u.login == login) {
            Some(u) => u.clone(),
            None => return Err((StatusCode::NOT_FOUND, "credentials-not-found").into()),
        };

        verify_password(&password, &user.password)?;

        let ws_id = ws_guard.id.clone();
        let ws_name = ws_guard.name.clone();
        let path = ws_guard.unit_tree.node_path(&user.node);

        (ws_id, ws_name, user, path)
    };

    let session = ClientSession {
        id: user.id,
        workspace,
        workspace_name,
        username: user.username,
        node: user.node,
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
