use crate::{middleware::*, repositories::*, services::*};
use ::axum::Json;
use ::indexmap::IndexMap;
use ::shared::{common::*, models::*, utils::*};

pub async fn get_quiz_record_base(
    session: &Session,
    id: impl Into<String>,
) -> Result<Json<QuizRecord>> {
    let quiz_record_arc = Store::find::<QuizRecord>(&session.workspace, id).await?;
    
    let quiz_record_base = {
        let quiz_record_guard = quiz_record_arc.read().await;
        quiz_record_guard.to_base()
    };

    Ok(Json(quiz_record_base))
}

pub async fn create_quiz_record(
    session: &Session,
    quiz_id: impl Into<String>,
    node: impl Into<String>,
) -> Result<Task> {
    let node_id = node.into();
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;

    let (nodes, path) = {
        let ws_guard = ws_arc.read().await;
        let nodes = ws_guard.unit_tree.node_descendants(&node_id);
        let path = ws_guard.unit_tree.node_path(&node_id);
        (nodes, path)
    };

    let mut students_vec =
        StudentRepository::list_by_filter(&session.workspace, Some(nodes)).await?;
    students_vec.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    let students = students_vec
        .into_iter()
        .map(|s| {
            (
                s.id.clone(),
                QuizRecordStudent {
                    id: s.id,
                    rank: s.rank,
                    name: s.name,
                    attempts: 0,
                    grade: 0,
                    answers: vec![],
                },
            )
        })
        .collect::<IndexMap<String, QuizRecordStudent>>();

    let quiz_arc = Store::find::<Quiz>(&session.workspace, quiz_id).await?;
    let record = {
        let quiz_guard = quiz_arc.read().await;

        let categories = quiz_guard
            .categories
            .values()
            .filter(|c| c.count > 0)
            .map(|c| {
                (
                    c.id.clone(),
                    QuizRecordCategory {
                        id: c.id.clone(),
                        name: c.name.clone(),
                    },
                )
            })
            .collect::<IndexMap<String, QuizRecordCategory>>();

        QuizRecord {
            id: safe_nanoid!(),
            workspace: session.workspace.clone(),
            name: quiz_guard.name.clone(),
            node: node_id.clone(),
            path,
            grade: quiz_guard.grade.clone(),
            categories,
            questions: Default::default(),
            answers: Default::default(),
            students,
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
