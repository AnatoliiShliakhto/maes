use crate::{middleware::*, repositories::*, services::*};
use ::axum::{
    Json,
    response::{IntoResponse, Response},
};
use ::indexmap::IndexMap;
use ::shared::{common::*, models::*, payloads::*, utils::*};
use ::std::collections::{HashMap, HashSet};
use std::ops::Index;

pub async fn get_quiz_record(session: &Session, id: impl Into<String>) -> Result<Response> {
    let quiz_record_arc = Store::find::<QuizRecord>(&session.workspace, id).await?;
    let quiz_record = { quiz_record_arc.read().await.clone() };
    Ok(Json(quiz_record).into_response())
}

pub async fn get_quiz_record_base(session: &Session, id: impl Into<String>) -> Result<Response> {
    let quiz_record_arc = Store::find::<QuizRecord>(&session.workspace, id).await?;
    let quiz_record_base = {
        let quiz_record_guard = quiz_record_arc.read().await;
        quiz_record_guard.to_base()
    };
    Ok(Json(quiz_record_base).into_response())
}

pub async fn create_quiz_record(session: &Session, payload: CreateTaskPayload) -> Result<Task> {
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;
    let CreateTaskPayload {
        id,
        node,
        name,
        path,
        categories,
    } = payload;

    let nodes = {
        let ws_guard = ws_arc.read().await;
        ws_guard.unit_tree.node_descendants(&node)
    };

    let students_fut = StudentRepository::list_by_filter(&session.workspace, Some(nodes));
    let quiz_fut = Store::find::<Quiz>(&session.workspace, id);
    let (mut students_vec, quiz_arc) = tokio::try_join!(students_fut, quiz_fut)?;

    students_vec.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    let mut students: IndexMap<String, QuizRecordStudent> =
        IndexMap::with_capacity(students_vec.len());
    for s in students_vec {
        let sid = s.id.clone();
        students.insert(
            sid,
            QuizRecordStudent {
                id: s.id,
                rank: s.rank,
                name: s.name,
                attempts: 0,
                grade: 0,
            },
        );
    }

    let record = {
        let quiz_guard = quiz_arc.read().await;

        let mut task_categories =
            IndexMap::<String, QuizRecordCategory>::with_capacity(categories.len());
        let mut total_count = 0_i64;
        let max_count = quiz_guard.categories.len();

        for category_req in categories {
            let Some(category) = quiz_guard.categories.get(&category_req.id) else {
                continue
            };
            let count = category_req.count.min(max_count);
            if count == 0 {
                continue;
            }
            total_count += count as i64;

            let category_id = category.id.clone();
            task_categories.insert(
                category_id.clone(),
                QuizRecordCategory {
                    id: category_id,
                    name: category.name.clone(),
                    count,
                },
            );
        }

        let answers = Grid::<HashMap<String, HashSet<String>>>::new(
            students.len(),
            task_categories.len(),
            Default::default(),
        );

        let results = Grid::<u8>::new(students.len(), task_categories.len(), 0_u8);

        QuizRecord {
            id: safe_nanoid!(),
            workspace: session.workspace.clone(),
            quiz: quiz_guard.id.clone(),
            name,
            node,
            path,
            attempts: quiz_guard.attempts,
            duration: quiz_guard.duration * total_count,
            grade: quiz_guard.grade.clone(),
            categories: task_categories,
            answers,
            students,
            results,
            metadata: Metadata::new(&session.username),
        }
    };

    let task = Task {
        id: record.id.clone(),
        workspace: record.workspace.clone(),
        kind: EntityKind::QuizRecord,
        name: record.name.clone(),
        node: record.node.clone(),
        path: record.path.clone(),
        progress: 0,
        metadata: record.metadata.clone(),
    };

    Store::upsert(record).await?;
    Ok(task)
}

pub async fn get_quiz_categories(session: &Session, id: impl Into<String>) -> Result<Vec<TaskCategory>> {
    let quiz_arc = Store::find::<Quiz>(&session.workspace, id).await?;

    let categories = {
        let quiz_guard = quiz_arc.read().await;

        quiz_guard.categories.values().map(|c| TaskCategory {
            id: c.id.clone(),
            name: c.name.clone(),
            count: c.count,
            total: c.questions.len(),
        }).collect::<Vec<_>>()
    };

    Ok(categories)
}

pub async fn get_quiz_activity_details(workspace: impl Into<String>, task_id: impl Into<String>, student: impl Into<String>) -> Result<Response> {
    let ws_id = workspace.into();
    let task_id = task_id.into();
    let student_id = student.into();

    let quiz_record_arc = Store::find::<QuizRecord>(&ws_id, task_id).await?;
    let activity = {
        let quiz_record_guard = quiz_record_arc.read().await;
        let student_idx = quiz_record_guard.students.get_index_of(&student_id).ok_or((StatusCode::NOT_FOUND, "student-not-found"))?;
        let scores = quiz_record_guard.results.get_row(student_idx);
        let score = if scores.is_empty() {
            0
        } else {
            let sum: i32 = scores.iter().map(|&&v| v as i32).sum();
            ((sum as f64) / (scores.len() as f64)).round() as u8
        };
        let student = quiz_record_guard.students.index(student_idx);
        let can_take = quiz_record_guard.attempts == 0 || quiz_record_guard.attempts > student.attempts;

        QuizActivityDetails {
            workspace: quiz_record_guard.workspace.clone(),
            quiz: quiz_record_guard.quiz.clone(),
            quiz_name: quiz_record_guard.name.clone(),
            duration: quiz_record_guard.duration,
            student: student.id.clone(),
            student_rank: student.rank.clone(),
            student_name: student.name.clone(),
            grade: student.grade,
            score,
            can_take,
        }
    };

    Ok(Json(activity).into_response())
}