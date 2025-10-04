use crate::{middleware::*, repositories::*, services::*};
use ::axum::{Json, extract::Path};
use ::shared::{common::*, models::*, payloads::*, utils::*};

pub async fn list_students(session: Session) -> Result<Json<Vec<Student>>> {
    let node_id = session.node.clone();
    list_students_by_node(session, Path(node_id)).await
}

pub async fn list_students_by_node(
    session: Session,
    Path(node_id): Path<String>,
) -> Result<Json<Vec<Student>>> {
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;

    let nodes = {
        let ws_guard = ws_arc.read().await;
        ws_guard.unit_tree.node_descendants(node_id)
    };

    let students = StudentRepository::list_by_filter(&session.workspace, Some(nodes)).await?;
    Ok(Json(students))
}

pub async fn add_students(
    session: Session,
    Path(node_id): Path<String>,
    Json(payload): Json<Vec<AddStudentPayload>>,
) -> Result<Json<Vec<Student>>> {
    let students_arc = Store::find::<Students>(&session.workspace, STUDENTS).await?;

    let students = payload
        .into_iter()
        .map(|s| Student {
            id: safe_nanoid!(),
            node: node_id.clone(),
            rank: s.rank,
            name: s.name,
        })
        .collect::<Vec<Student>>();

    let snapshot = {
        let mut students_guard = students_arc.write().await;
        for student in students.clone() {
            students_guard.insert(student.id.clone(), student);
        }

        students_guard.sort_unstable_by(|_, a, _, b| a.name.cmp(&b.name));
        students_guard.clone()
    };

    Store::upsert(snapshot).await?;

    Ok(Json(students))
}

pub async fn remove_students(session: Session, Json(payload): Json<Vec<String>>) -> Result<()> {
    let payload = if payload.is_empty() {
        None
    } else {
        Some(payload)
    };
    StudentRepository::batch_remove(&session.workspace, payload, None).await
}

pub async fn remove_students_by_node(
    session: Session,
    Path(node_id): Path<String>,
    Json(payload): Json<Vec<String>>,
) -> Result<()> {
    let ws_arc = Store::find::<Workspace>(&session.workspace, &session.workspace).await?;

    let payload = if payload.is_empty() {
        None
    } else {
        Some(payload)
    };
    let nodes = {
        let ws_guard = ws_arc.read().await;
        ws_guard.unit_tree.node_descendants(node_id)
    };

    StudentRepository::batch_remove(&session.workspace, payload, Some(nodes)).await
}
