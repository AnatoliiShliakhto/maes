use crate::{middleware::*, repositories::*, services::*};
use ::axum::{
    Json,
    extract::Path,
    response::{IntoResponse, Response},
};
use ::indexmap::IndexMap;
use ::shared::{common::*, models::*, payloads::*};
use ::std::collections::HashSet;

pub async fn get_quiz(session: Session, Path(quiz_id): Path<String>) -> Result<Json<Quiz>> {
    session.checked_supervisor()?;
    let quiz_arc = Store::find::<Quiz>(&session.workspace, quiz_id).await?;
    let snapshot = { quiz_arc.read().await.clone() };
    Ok(Json(snapshot))
}

pub async fn create_quiz(
    session: Session,
    Json(payload): Json<CreateQuizPayload>,
) -> Result<Json<Quiz>> {
    session.checked_admin()?;
    let CreateQuizPayload { name, node } = payload;

    let quiz = Quiz {
        id: safe_nanoid!(),
        name,
        workspace: session.workspace.clone(),
        node,
        metadata: Metadata::new(&session.username),
        ..Default::default()
    };

    EntityRepository::upsert(&session.workspace, quiz.to_entity()).await?;
    Store::upsert(quiz.clone()).await?;
    Ok(Json(quiz))
}

pub async fn update_quiz(
    session: Session,
    Path(quiz_id): Path<String>,
    Json(payload): Json<UpdateQuizPayload>,
) -> Result<Json<Quiz>> {
    session.checked_admin()?;
    let UpdateQuizPayload {
        name,
        node,
        attempts,
        duration,
        grade,
        categories,
    } = payload;
    let mut quiz_arc = Store::find::<Quiz>(&session.workspace, quiz_id).await?;
    let snapshot = {
        let mut quiz_guard = quiz_arc.write().await;
        quiz_guard.name = name;
        quiz_guard.node = node;
        quiz_guard.attempts = attempts;
        quiz_guard.duration = duration;
        quiz_guard.grade = grade;
        if !categories.is_empty() {
            quiz_guard.categories = categories.into_iter().map(|c| (c.id.clone(), c)).collect();
            quiz_guard
                .categories
                .sort_unstable_by(|_, a, _, b| a.order.cmp(&b.order).then(a.name.cmp(&b.name)));
        }
        quiz_guard.metadata.update(&session.username);

        quiz_guard.clone()
    };

    let base = snapshot.to_base();
    EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
    Store::upsert(snapshot).await?;
    Ok(Json(base))
}

pub async fn delete_quiz(session: Session, Path(quiz_id): Path<String>) -> Result<Json<String>> {
    session.checked_admin()?;
    let quiz_arc = Store::find::<Quiz>(&session.workspace, &quiz_id).await?;
    if quiz_id != quiz_arc.read().await.workspace {
        Err((StatusCode::FORBIDDEN, "forbidden"))?
    }

    EntityRepository::delete(&session.workspace, Some(quiz_id.to_string()), None).await?;
    Store::delete(&session.workspace, &quiz_id).await?;
    Ok(Json(quiz_id))
}

pub async fn create_quiz_category(
    session: Session,
    Path(quiz_id): Path<String>,
    Json(payload): Json<UpdateQuizCategoryPayload>,
) -> Result<Json<QuizCategory>> {
    update_quiz_category(session, Path((quiz_id, safe_nanoid!())), Json(payload)).await
}

pub async fn update_quiz_category(
    session: Session,
    Path((quiz_id, category_id)): Path<(String, String)>,
    Json(payload): Json<UpdateQuizCategoryPayload>,
) -> Result<Json<QuizCategory>> {
    session.checked_admin()?;
    let quiz_arc = Store::find::<Quiz>(&session.workspace, quiz_id).await?;
    let UpdateQuizCategoryPayload {
        name,
        important,
        count,
        order,
    } = payload;

    let (snapshot, category) = {
        let mut quiz_guard = quiz_arc.write().await;
        let category = if let Some(category) = quiz_guard.categories.get_mut(&category_id) {
            category.name = name;
            category.important = important;
            category.count = count;
            category.order = order;

            category.clone()
        } else {
            let category = QuizCategory {
                id: category_id,
                name,
                important,
                count,
                order,
                ..Default::default()
            };

            quiz_guard
                .categories
                .insert(category.id.clone(), category.clone());

            category.to_base()
        };
        quiz_guard
            .categories
            .sort_unstable_by(|_, a, _, b| a.order.cmp(&b.order).then(a.name.cmp(&b.name)));
        quiz_guard.metadata.update(&session.username);

        (quiz_guard.clone(), category)
    };

    EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
    Store::upsert(snapshot).await?;
    Ok(Json(category))
}

pub async fn delete_quiz_category(
    session: Session,
    Path((quiz_id, category_id)): Path<(String, String)>,
) -> Result<Json<String>> {
    session.checked_admin()?;
    let quiz_arc = Store::find::<Quiz>(&session.workspace, quiz_id).await?;

    let snapshot = {
        let mut quiz_guard = quiz_arc.write().await;

        quiz_guard.categories.shift_remove(&category_id);
        quiz_guard.metadata.update(&session.username);

        quiz_guard.clone()
    };

    EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
    Store::upsert(snapshot).await?;
    Ok(Json(category_id))
}

pub async fn create_quiz_question(
    session: Session,
    Path((quiz_id, category_id)): Path<(String, String)>,
    Json(payload): Json<UpdateQuizQuestionPayload>,
) -> Result<Json<QuizQuestion>> {
    update_quiz_question(
        session,
        Path((quiz_id, category_id, safe_nanoid!())),
        Json(payload),
    )
    .await
}

pub async fn update_quiz_question(
    session: Session,
    Path((quiz_id, category_id, question_id)): Path<(String, String, String)>,
    Json(payload): Json<UpdateQuizQuestionPayload>,
) -> Result<Json<QuizQuestion>> {
    session.checked_admin()?;
    let quiz_arc = Store::find::<Quiz>(&session.workspace, quiz_id).await?;
    let UpdateQuizQuestionPayload { name, img, answers } = payload;
    let question = QuizQuestion {
        id: question_id,
        name,
        img,
        answers: answers
            .into_iter()
            .map(|a| (a.id.clone(), a))
            .collect::<IndexMap<String, QuizAnswer>>(),
    };

    let snapshot = {
        let mut quiz_guard = quiz_arc.write().await;
        let category = quiz_guard
            .categories
            .get_mut(&category_id)
            .ok_or((StatusCode::NOT_FOUND, "entity-not-found"))?;

        category
            .questions
            .insert(question.id.clone(), question.clone());
        category
            .questions
            .sort_unstable_by(|_, a, _, b| a.name.cmp(&b.name));
        quiz_guard.metadata.update(&session.username);

        quiz_guard.clone()
    };

    EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
    Store::upsert(snapshot).await?;
    Ok(Json(question))
}

pub async fn delete_quiz_question(
    session: Session,
    Path((quiz_id, category_id, question_id)): Path<(String, String, String)>,
) -> Result<Json<String>> {
    session.checked_admin()?;
    let quiz_arc = Store::find::<Quiz>(&session.workspace, quiz_id).await?;

    let snapshot = {
        let mut quiz_guard = quiz_arc.write().await;
        let category = quiz_guard
            .categories
            .get_mut(&category_id)
            .ok_or((StatusCode::NOT_FOUND, "entity-not-found"))?;

        category.questions.shift_remove(&question_id);
        quiz_guard.metadata.update(&session.username);

        quiz_guard.clone()
    };

    EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
    Store::upsert(snapshot).await?;
    Ok(Json(question_id))
}

pub async fn validate_quiz_images(
    session: &Session,
    quiz_id: impl Into<String>,
) -> Result<Response> {
    session.checked_admin()?;
    let quiz_id = quiz_id.into();

    let image_set = ImageService::get_entity_images(&session.workspace, &quiz_id).await?;

    let quiz_arc = Store::find::<Quiz>(&session.workspace, &quiz_id).await?;
    let mut quiz_image_set = HashSet::with_capacity(image_set.len());

    let snapshot = {
        let mut quiz = quiz_arc.write().await;

        for category in quiz.categories.values_mut() {
            for question in category.questions.values_mut() {
                if question.img && !image_set.contains(&question.id) {
                    question.img = false;
                }
                if question.img {
                    quiz_image_set.insert(question.id.clone());
                }

                for answer in question.answers.values_mut() {
                    if answer.img && !image_set.contains(&answer.id) {
                        answer.img = false;
                    }
                    if answer.img {
                        quiz_image_set.insert(answer.id.clone());
                    }
                }
            }
        }

        quiz.metadata.update(&session.username);
        quiz.clone()
    };

    let orphan_files: HashSet<String> = image_set.difference(&quiz_image_set).cloned().collect();

    EntityRepository::upsert(&session.workspace, snapshot.to_entity()).await?;
    Store::upsert(snapshot.clone()).await?;
    if !orphan_files.is_empty() {
        ImageService::batch_remove(&session.workspace, quiz_id, orphan_files).await?;
    }

    Ok(Json(snapshot).into_response())
}
