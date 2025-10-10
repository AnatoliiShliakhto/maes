use crate::{handlers::*, services::*};
use ::axum::{Json, extract::Path, response::Response};
use ::serde::Deserialize;
use ::serde_json::Value;
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

pub async fn get_activity(Path((workspace, task_id)): Path<(String, String)>) -> Result<Response> {
    get_activity_with_student(Path((workspace, task_id, "".to_string()))).await
}

pub async fn get_activity_with_student(
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
        EntityKind::QuizRecord => get_quiz_activity(workspace, task_id, student_id).await,
        EntityKind::SurveyRecord => get_survey_activity(workspace, task_id).await,
        _ => Err((StatusCode::NOT_FOUND, "task-not-found"))?,
    }
}

pub async fn update_activity(Json(payload): Json<Value>) -> Result<()> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Variants {
        QuizActivity(QuizActivity),
        SurveyActivity(SurveyRecord),
    }

    let payload: Variants =
        serde_json::from_value(payload).map_err(|_| (StatusCode::BAD_REQUEST, "invalid-payload"))?;

    match payload {
        Variants::QuizActivity(quiz) => update_quiz_activity(quiz).await,
        Variants::SurveyActivity(survey) => update_survey_activity(survey).await,
    }
}
