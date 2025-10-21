use crate::{handlers::*, middleware::*, repositories::*, services::*};
use ::axum::{Json, extract::Path, response::Response};
use ::shared::{common::*, models::*, payloads::*};
use ::std::str::FromStr;

pub async fn list_tasks(session: Session) -> Result<Json<Vec<Task>>> {
    let nodes = session.nodes().await?;
    let tasks = TaskRepository::list_by_filter(&session.workspace, None, nodes).await?;
    Ok(Json(tasks))
}

pub async fn get_task(
    session: Session,
    Path((kind, task_id)): Path<(String, String)>,
) -> Result<Response> {
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;
    match kind {
        EntityKind::QuizRecord => get_quiz_record_base(&session, task_id).await,
        EntityKind::SurveyRecord => get_survey_record_base(&session, task_id).await,
        _ => Err((StatusCode::BAD_REQUEST, "bad-request"))?,
    }
}

pub async fn create_task(
    session: Session,
    Path(kind): Path<String>,
    Json(payload): Json<CreateTaskPayload>,
) -> Result<Json<Task>> {
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;

    let task = match kind {
        EntityKind::Quiz => create_quiz_record(&session, payload).await?,
        EntityKind::Survey => create_survey_record(&session, payload).await?,
        _ => Err((StatusCode::BAD_REQUEST, "bad-request"))?,
    };

    TaskRepository::upsert(&session.workspace, task.clone()).await?;
    Ok(Json(task))
}

pub async fn delete_task(
    session: Session,
    Path((kind, task_id)): Path<(String, String)>,
) -> Result<()> {
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;
    if kind != EntityKind::QuizRecord && kind != EntityKind::SurveyRecord {
        Err((StatusCode::BAD_REQUEST, "bad-request"))?
    }

    Store::delete(&session.workspace, &task_id).await?;
    TaskRepository::delete(&session.workspace, Some(task_id), None).await
}

pub async fn get_task_categories(
    session: Session,
    Path((kind, id)): Path<(String, String)>,
) -> Result<Json<Vec<TaskCategory>>> {
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;

    let categories = match kind {
        EntityKind::Quiz => get_quiz_categories(&session, id).await?,
        EntityKind::Survey => get_survey_categories(&session, id).await?,
        _ => Err((StatusCode::BAD_REQUEST, "bad-request"))?,
    };

    Ok(Json(categories))
}

pub async fn finish_task(session: Session, Path(task_id): Path<String>) -> Result<Json<String>> {
    let task = TaskRepository::get(&session.workspace, &task_id).await?;
    if task.kind != EntityKind::QuizRecord && task.kind != EntityKind::SurveyRecord {
        Err((StatusCode::BAD_REQUEST, "bad-request"))?
    }
    let mut metadata = task.metadata;
    metadata.update(&session.username);
    let entity = match task.kind {
        EntityKind::QuizRecord |
        EntityKind::SurveyRecord => Entity {
            id: task.id,
            name: task.name,
            kind: task.kind,
            node: task.node,
            path: task.path,
            metadata,
        },
        _ => Err((StatusCode::BAD_REQUEST, "not-found"))?,
    };
    TaskRepository::delete(&session.workspace, Some(task_id.clone()), None).await?;
    EntityRepository::upsert(&session.workspace, entity).await?;
    Ok(Json(task_id))
}