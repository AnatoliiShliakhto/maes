use crate::{handlers::*, services::*};
use ::axum::{extract::Path, response::Response};
use ::shared::{common::*, models::*};

pub async fn get_activity_details(
    Path((workspace, task_id)): Path<(String, String)>,
) -> Result<Response> {
    get_activity_details_with_student(Path((workspace, task_id, "".to_string()))).await
}

pub async fn get_activity_details_with_student(
    Path((workspace, task_id, student_id)): Path<(String, String, String)>,
) -> Result<Response> {
    let tasks_arc = Store::find::<Tasks>(&workspace, TASKS).await?;

    let kind = {
        let tasks_guard = tasks_arc.read().await;
        let task_guard = tasks_guard
            .get(&task_id)
            .ok_or((StatusCode::NOT_FOUND, "task-not-found"))?;
        task_guard.kind
    };

    match kind {
        EntityKind::QuizRecord => get_quiz_activity_details(workspace, task_id, student_id).await,
        EntityKind::SurveyRecord => get_survey_activity_details(workspace, task_id).await,
        _ => Err((StatusCode::NOT_FOUND, "task-not-found"))?,
    }
}
