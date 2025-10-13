use crate::{handlers::*, middleware::*, services::*};
use ::axum::{Json, extract::Path, response::Response};
use ::shared::{common::*, models::*, payloads::*};
use ::std::str::FromStr;

pub async fn add_image(
    session: Session,
    Path((entity_id, item_id)): Path<(String, String)>,
    Json(payload): Json<AddImagePayload>,
) -> Result<()> {
    session.checked_admin()?;
    let AddImagePayload { path } = payload;

    ImageService::convert_and_save(path, &session.workspace, entity_id, item_id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "image-save-error"))?;

    Ok(())
}

pub async fn remove_image(
    session: Session,
    Path((entity_id, item_id)): Path<(String, String)>,
) -> Result<()> {
    session.checked_admin()?;
    ImageService::remove(&session.workspace, entity_id, item_id)
        .await
        .ok();
    Ok(())
}

pub async fn validate_images(
    session: Session,
    Path((kind, entity_id)): Path<(String, String)>,
) -> Result<Response> {
    session.checked_admin()?;
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;

    match kind {
        EntityKind::Quiz => validate_quiz_images(&session, entity_id).await,
        _ => Err((StatusCode::BAD_REQUEST, "bad-request"))?,
    }
}

pub async fn copy_images(session: Session, Json(payload): Json<CopyImagesPayload>) -> Result<()> {
    session.checked_admin()?;
    let CopyImagesPayload {
        source_workspace,
        source_entity,
        destination_workspace,
        destination_entity,
    } = payload;
    ImageService::copy_images(
        source_workspace,
        source_entity,
        destination_workspace,
        destination_entity,
    )
}
