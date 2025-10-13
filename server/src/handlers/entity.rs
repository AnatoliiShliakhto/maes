use crate::{middleware::*, repositories::*, services::*};
use ::axum::{
    Json,
    extract::Path,
    response::{IntoResponse, Response},
};
use ::shared::{common::*, models::*, utils::*};
use ::std::str::FromStr;

pub async fn list_entities(
    session: Session,
    Path(kind): Path<String>,
) -> Result<Json<Vec<Entity>>> {
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;
    let entities =
        EntityRepository::list_by_filter(&session.workspace, Some(vec![kind]), None).await?;

    Ok(Json(entities))
}

pub async fn list_entities_by_node(
    session: Session,
    Path((kind, node_id)): Path<(String, String)>,
) -> Result<Json<Vec<Entity>>> {
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;

    let nodes = {
        let ws_guard = ws_arc.read().await;
        match kind {
            EntityKind::Quiz => ws_guard.quiz_tree.node_descendants(node_id),
            EntityKind::Survey => ws_guard.survey_tree.node_descendants(node_id),
            EntityKind::Checklist => ws_guard.checklist_tree.node_descendants(node_id),
            _ => vec![],
        }
    };
    if nodes.is_empty() {
        return Ok(Json(vec![]));
    }

    let entities =
        EntityRepository::list_by_filter(&session.workspace, Some(vec![kind]), Some(nodes)).await?;
    Ok(Json(entities))
}

pub async fn delete_entity(
    session: Session,
    Path((kind, entity_id)): Path<(String, String)>,
) -> Result<Json<String>> {
    session.checked_admin()?;
    let _kind =
        EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;

    EntityRepository::delete(&session.workspace, Some(entity_id.clone()), None).await?;
    Store::delete(&session.workspace, &entity_id).await?;
    ImageService::remove_entities(&session.workspace, vec![entity_id.clone()]).await?;
    Ok(Json(entity_id))
}

pub async fn get_entity_payload(
    session: Session,
    Path((kind, id)): Path<(String, String)>,
) -> Result<Response> {
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;

    match kind {
        EntityKind::QuizRecord => {
            let quiz_record = Store::find::<QuizRecord>(&session.workspace, id)
                .await?
                .read()
                .await
                .clone();
            Ok(Json(quiz_record).into_response())
        }
        EntityKind::SurveyRecord => {
            let survey_record = Store::find::<SurveyRecord>(&session.workspace, id)
                .await?
                .read()
                .await
                .clone();
            Ok(Json(survey_record).into_response())
        }
        _ => Err((StatusCode::NOT_FOUND, "entity-not-found"))?,
    }
}
