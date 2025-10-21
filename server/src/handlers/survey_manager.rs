use crate::{middleware::*, repositories::*, services::*};
use ::axum::{Json, extract::Path};
use ::indexmap::IndexMap;
use ::shared::{common::*, models::*, payloads::*};

pub async fn get_survey(session: Session, Path(survey_id): Path<String>) -> Result<Json<Survey>> {
    session.checked_supervisor()?;
    let survey_arc = Store::find::<Survey>(&session.workspace, survey_id).await?;
    let snapshot = { survey_arc.read().await.clone() };
    Ok(Json(snapshot))
}

pub async fn create_survey(
    session: Session,
    Json(payload): Json<CreateSurveyPayload>,
) -> Result<Json<Survey>> {
    session.checked_admin()?;
    let CreateSurveyPayload { name, node } = payload;

    let survey = Survey {
        id: safe_nanoid!(),
        name,
        workspace: session.workspace.clone(),
        node,
        metadata: Metadata::new(&session.username),
        ..Default::default()
    };

    EntityRepository::upsert(&session.workspace, survey.to_entity()).await?;
    Store::upsert(survey.clone()).await?;
    Ok(Json(survey))
}

pub async fn update_survey(
    session: Session,
    Path(survey_id): Path<String>,
    Json(payload): Json<UpdateSurveyPayload>,
) -> Result<Json<Survey>> {
    session.checked_admin()?;
    let UpdateSurveyPayload {
        name,
        node,
        categories,
    } = payload;
    let survey_arc = Store::find::<Survey>(&session.workspace, survey_id).await?;
    let snapshot = {
        let mut survey_guard = survey_arc.write().await;
        survey_guard.name = name;
        survey_guard.node = node;
        if !categories.is_empty() {
            let categories = categories
                .into_iter()
                .map(|c| (c.id.clone(), c))
                .collect::<IndexMap<String, SurveyCategory>>();
            survey_guard.categories.extend(categories);
        }
        survey_guard.metadata.update(&session.username);

        survey_guard.clone()
    };

    let base = snapshot.to_base();
    EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
    Store::upsert(snapshot).await?;
    Ok(Json(base))
}

pub async fn delete_survey(
    session: Session,
    Path(survey_id): Path<String>,
) -> Result<Json<String>> {
    session.checked_admin()?;
    let survey_arc = Store::find::<Survey>(&session.workspace, &survey_id).await?;
    if survey_id != survey_arc.read().await.workspace {
        Err((StatusCode::FORBIDDEN, "forbidden"))?
    }

    EntityRepository::delete(&session.workspace, Some(survey_id.to_string()), None).await?;
    Store::delete(&session.workspace, &survey_id).await?;
    Ok(Json(survey_id))
}

pub async fn create_survey_category(
    session: Session,
    Path(survey_id): Path<String>,
    Json(payload): Json<UpdateSurveyCategoryPayload>,
) -> Result<Json<SurveyCategory>> {
    update_survey_category(session, Path((survey_id, safe_nanoid!())), Json(payload)).await
}

pub async fn update_survey_category(
    session: Session,
    Path((survey_id, category_id)): Path<(String, String)>,
    Json(payload): Json<UpdateSurveyCategoryPayload>,
) -> Result<Json<SurveyCategory>> {
    session.checked_admin()?;
    let survey_arc = Store::find::<Survey>(&session.workspace, survey_id).await?;
    let UpdateSurveyCategoryPayload {
        name,
        order,
        answers,
        questions,
    } = payload;

    let answers = answers.into_iter().map(|a| (a.id.clone(), a)).collect();
    let questions = questions.into_iter().map(|q| (q.id.clone(), q)).collect();

    let (snapshot, category) = {
        let mut survey_guard = survey_arc.write().await;
        let category = if let Some(category) = survey_guard.categories.get_mut(&category_id) {
            category.name = name;
            category.order = order;
            category.answers = answers;
            category.questions = questions;

            category.clone()
        } else {
            let category = SurveyCategory {
                id: category_id,
                name,
                order,
                answers,
                questions,
                ..Default::default()
            };

            survey_guard
                .categories
                .insert(category.id.clone(), category.clone());

            category.clone()
        };
        survey_guard
            .categories
            .sort_unstable_by(|_, a, _, b| a.order.cmp(&b.order).then(a.name.cmp(&b.name)));
        survey_guard.metadata.update(&session.username);

        (survey_guard.clone(), category)
    };

    EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
    Store::upsert(snapshot).await?;
    Ok(Json(category))
}

pub async fn delete_survey_category(
    session: Session,
    Path((survey_id, category_id)): Path<(String, String)>,
) -> Result<Json<String>> {
    session.checked_admin()?;
    let survey_arc = Store::find::<Survey>(&session.workspace, survey_id).await?;

    let snapshot = {
        let mut survey_guard = survey_arc.write().await;

        survey_guard.categories.shift_remove(&category_id);
        survey_guard.metadata.update(&session.username);

        survey_guard.clone()
    };

    EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
    Store::upsert(snapshot).await?;
    Ok(Json(category_id))
}
