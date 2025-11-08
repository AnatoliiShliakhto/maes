use crate::{middleware::*, repositories::*, services::*};
use ::axum::{
    Json,
    extract::Path,
    response::{IntoResponse, Response},
};
use ::shared::{common::*, models::*, utils::*};
use ::std::str::FromStr;
use shared::payloads::UpdateEntityPayload;

pub async fn list_reports(session: Session) -> Result<Json<Vec<Entity>>> {
    let kinds = vec![EntityKind::QuizRecord, EntityKind::SurveyRecord];
    let nodes = session.nodes().await?;

    let entities =
        EntityRepository::list_by_filter(&session.workspace, Some(kinds), None, nodes).await?;

    Ok(Json(entities))
}

pub async fn delete_entities(session: Session, Json(entities): Json<Vec<String>>) -> Result<()> {
    //todo: check entities kinds
    EntityRepository::batch_remove(&session.workspace, Some(entities), None).await?;
    Ok(())
}

pub async fn list_entities(
    session: Session,
    Path(kind): Path<String>,
) -> Result<Json<Vec<Entity>>> {
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;
    let entities = if kind == EntityKind::Workspace {
        EntityRepository::list_by_filter(&session.workspace, Some(vec![kind]), None, None).await?
    } else {
        let nodes = session.nodes().await?;
        EntityRepository::list_by_filter(&session.workspace, Some(vec![kind]), None, nodes).await?
    };

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
        EntityRepository::list_by_filter(&session.workspace, Some(vec![kind]), None, Some(nodes))
            .await?;
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

pub async fn update_entity(
    session: Session,
    Path((kind, id)): Path<(String, String)>,
    Json(payload): Json<UpdateEntityPayload>,
) -> Result<()> {
    let kind = EntityKind::from_str(&kind).map_err(|_| (StatusCode::BAD_REQUEST, "bad-request"))?;

    match kind {
        EntityKind::QuizRecord => {
            let quiz_rec_arc = Store::find::<QuizRecord>(&session.workspace, id).await?;
            let snapshot = {
                let mut quiz_rec_guard = quiz_rec_arc.write().await;
                quiz_rec_guard.name = payload.name;
                quiz_rec_guard.path = payload.path;
                quiz_rec_guard.clone()
            };
            EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
            Store::upsert(snapshot).await?;
            Ok(())
        }
        EntityKind::SurveyRecord => {
            let survey_rec_arc = Store::find::<SurveyRecord>(&session.workspace, id).await?;
            let snapshot = {
                let mut survey_rec_guard = survey_rec_arc.write().await;
                survey_rec_guard.name = payload.name;
                survey_rec_guard.path = payload.path;
                survey_rec_guard.clone()
            };
            EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
            Store::upsert(snapshot).await?;
            Ok(())
        }
        _ => Err((StatusCode::NOT_FOUND, "entity-not-found"))?,
    }
}

pub async fn merge_entities(
    session: Session,
    Json(payload): Json<Vec<String>>,
) -> Result<Json<Entity>> {
    let entities =
        EntityRepository::list_by_filter(&session.workspace, None, Some(payload), None).await?;

    let Some(entity) = entities.first() else {
        Err((StatusCode::CONFLICT, "entities-merge-failed"))?
    };
    let entities = match entity.kind {
        EntityKind::QuizRecord => {
            let mut entities = entities
                .iter()
                .filter(|e| e.kind == EntityKind::QuizRecord)
                .collect::<Vec<_>>();
            entities.sort_by(|a, b| a.path.cmp(&b.path));
            entities.dedup_by(|a, b| a.id == b.id);
            entities.iter().map(|e| e.id.clone()).collect::<Vec<_>>()
        }
        EntityKind::SurveyRecord => entities
            .iter()
            .filter(|e| e.kind == EntityKind::SurveyRecord)
            .map(|e| e.id.clone())
            .collect::<Vec<_>>(),
        _ => vec![],
    };

    if entities.len() < 2 {
        Err((StatusCode::CONFLICT, "entities-merge-failed"))?
    }

    let entity = match entity.kind {
        EntityKind::QuizRecord => merge_quiz_records(&session, entities).await?,
        EntityKind::SurveyRecord => merge_survey_records(&session, entities).await?,
        _ => Err((StatusCode::CONFLICT, "entities-merge-failed"))?,
    };

    Ok(Json(entity))
}

async fn merge_quiz_records(session: &Session, entities: Vec<String>) -> Result<Entity> {
    let mut merge_count = 1;
    let unit_tree = Store::find::<Workspace>(&session.workspace, &session.workspace)
        .await?
        .read()
        .await
        .unit_tree
        .clone();
    let mut merge = Store::find::<QuizRecord>(&session.workspace, &entities[0])
        .await?
        .read()
        .await
        .clone();
    for entity in entities.iter().skip(1) {
        let quiz_rec_arc = Store::find::<QuizRecord>(&session.workspace, entity).await?;
        let quiz_rec_guard = quiz_rec_arc.read().await;
        if merge.id == quiz_rec_guard.id
            || merge.quiz != quiz_rec_guard.quiz
            || merge.categories.keys().collect::<Vec<_>>()
                != quiz_rec_guard.categories.keys().collect::<Vec<_>>()
        {
            continue;
        }
        merge.students.extend(quiz_rec_guard.students.clone());
        merge.answers.extend_rows(&quiz_rec_guard.answers);
        merge.results.extend_rows(&quiz_rec_guard.results);

        let merge_path = unit_tree.node_path_ids(&merge.node);
        let other_path = unit_tree.node_path_ids(&quiz_rec_guard.node);
        if let Some(common_node) = find_last_common(&merge_path, &other_path) {
            merge.path = unit_tree.node_path(&common_node);
            merge.node = common_node;
        }
        merge_count += 1;
    }

    if merge_count == 1 {
        Err((StatusCode::CONFLICT, "entities-merge-failed"))?
    }
    merge.id = safe_nanoid!();
    merge.metadata.update(&session.username);

    let entity = merge.to_entity();
    Store::upsert(merge).await?;
    EntityRepository::upsert(&session.workspace, entity.clone()).await?;

    Ok(entity)
}

async fn merge_survey_records(session: &Session, entities: Vec<String>) -> Result<Entity> {
    let mut merge_count = 1;
    let unit_tree = Store::find::<Workspace>(&session.workspace, &session.workspace)
        .await?
        .read()
        .await
        .unit_tree
        .clone();
    let mut merge = Store::find::<SurveyRecord>(&session.workspace, &entities[0])
        .await?
        .read()
        .await
        .clone();
    for entity in entities.iter().skip(1) {
        let survey_rec_arc = Store::find::<SurveyRecord>(&session.workspace, entity).await?;
        let survey_rec_guard = survey_rec_arc.read().await;
        if merge.id == survey_rec_guard.id
            || merge.survey != survey_rec_guard.survey
            || merge.categories.keys().collect::<Vec<_>>()
                != survey_rec_guard.categories.keys().collect::<Vec<_>>()
        {
            continue;
        }

        for category in merge.categories.values_mut() {
            if let Some(other) = survey_rec_guard.categories.get(&category.id) {
                category.results.merge(&other.results)
            }
        }

        let merge_path = unit_tree.node_path_ids(&merge.node);
        let other_path = unit_tree.node_path_ids(&survey_rec_guard.node);
        if let Some(common_node) = find_last_common(&merge_path, &other_path) {
            merge.path = unit_tree.node_path(&common_node);
            merge.node = common_node;
        }
        merge.total += survey_rec_guard.total;
        merge_count += 1;
    }

    if merge_count == 1 {
        Err((StatusCode::CONFLICT, "entities-merge-failed"))?
    }
    merge.id = safe_nanoid!();
    merge.metadata.update(&session.username);

    let entity = merge.to_entity();
    Store::upsert(merge).await?;
    EntityRepository::upsert(&session.workspace, entity.clone()).await?;

    Ok(entity)
}
