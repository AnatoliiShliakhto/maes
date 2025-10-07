use crate::{middleware::*, services::*};
use ::axum::{
    Json,
    response::{IntoResponse, Response},
};
use ::indexmap::IndexMap;
use ::shared::{common::*, models::*, payloads::*};

pub async fn get_survey_record(session: &Session, id: impl Into<String>) -> Result<Response> {
    let survey_record_arc = Store::find::<SurveyRecord>(&session.workspace, id).await?;
    let survey_record = { survey_record_arc.read().await.clone() };
    Ok(Json(survey_record).into_response())
}

pub async fn get_survey_record_base(session: &Session, id: impl Into<String>) -> Result<Response> {
    let survey_record_arc = Store::find::<SurveyRecord>(&session.workspace, id).await?;

    let survey_record_base = {
        let survey_record_guard = survey_record_arc.read().await;
        survey_record_guard.to_base()
    };

    Ok(Json(survey_record_base).into_response())
}

pub async fn create_survey_record(session: &Session, payload: CreateTaskPayload) -> Result<Task> {
    let CreateTaskPayload {
        id,
        node,
        name,
        path,
        categories,
    } = payload;

    let survey_arc = Store::find::<Survey>(&session.workspace, id).await?;

    let record = {
        let survey_guard = survey_arc.read().await;

        let mut task_categories =
            IndexMap::<String, SurveyRecordCategory>::with_capacity(categories.len());

        for cat in categories {
            let Some(c) = survey_guard.categories.get(&cat.id) else {
                continue;
            };

            if c.questions.is_empty() {
                continue;
            }

            let questions_count = c.questions.len();
            let answers_count = c.answers.len().max(1);

            let category_id = c.id.clone();
            task_categories.insert(
                category_id.clone(),
                SurveyRecordCategory {
                    id: category_id,
                    name: c.name.clone(),
                    questions: c.questions.clone(),
                    answers: c.answers.clone(),
                    results: Grid::<usize>::new(questions_count, answers_count, 0),
                },
            );
        }

        SurveyRecord {
            id: safe_nanoid!(),
            workspace: session.workspace.clone(),
            survey: survey_guard.id.clone(),
            name,
            node,
            path,
            total: 0,
            categories: task_categories,
            metadata: Metadata::new(&session.username),
        }
    };

    let task = Task {
        id: record.id.clone(),
        workspace: record.workspace.clone(),
        kind: EntityKind::SurveyRecord,
        name: record.name.clone(),
        node: record.node.clone(),
        path: record.path.clone(),
        progress: 0,
        metadata: record.metadata.clone(),
    };

    Store::upsert(record).await?;
    Ok(task)
}

pub async fn get_survey_categories(session: &Session, id: impl Into<String>) -> Result<Vec<TaskCategory>> {
    let survey_arc = Store::find::<Survey>(&session.workspace, id).await?;

    let categories = {
        let survey_guard = survey_arc.read().await;

        survey_guard.categories.values().map(|c| TaskCategory {
            id: c.id.clone(),
            name: c.name.clone(),
            count: 0,
            total: 0,
        }).collect::<Vec<_>>()
    };

    Ok(categories)
}

pub async fn get_survey_activity_details(workspace: impl Into<String>, task_id: impl Into<String>) -> Result<Response> {
    let ws_id = workspace.into();
    let task_id = task_id.into();
    
    let survey_rec_arc = Store::find::<SurveyRecord>(&ws_id, task_id).await?;
    let activity = {
        let survey_record_guard = survey_rec_arc.read().await;
        
        SurveyActivityDetails {
            workspace: survey_record_guard.workspace.clone(),
            survey: survey_record_guard.survey.clone(),
            survey_name: survey_record_guard.name.clone(),
        }
    };
    
    Ok(Json(activity).into_response())
}