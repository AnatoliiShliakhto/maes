use crate::services::*;
use ::shared::{common::*, models::*};
use ::std::collections::HashSet;

pub struct StudentRepository;

impl StudentRepository {
    pub async fn init(workspace: impl AsRef<str>) -> Result<()> {
        let ws_id = workspace.as_ref();
        if !Store::get_path(ws_id, STUDENTS).exists() {
            let students = Students::new(ws_id);
            Store::upsert(students).await?;
        };
        Ok(())
    }

    pub async fn list_by_filter(
        workspace: impl Into<String>,
        nodes: Option<Vec<String>>,
    ) -> Result<Vec<Student>> {
        let students_arc = Store::find::<Students>(workspace, STUDENTS).await?;
        let nodes = nodes.map(|v| v.into_iter().collect::<HashSet<String>>());

        let snapshot = {
            let students_guard = students_arc.read().await;
            let mut out = Vec::with_capacity(students_guard.len());
            for s in students_guard.values() {
                if nodes.as_ref().map_or(true, |set| set.contains(&s.node)) {
                    out.push(s.clone());
                }
            }
            out
        };

        Ok(snapshot)
    }

    pub async fn add(workspace: impl Into<String>, payload: Vec<Student>) -> Result<()> {
        let students_arc = Store::find::<Students>(workspace, STUDENTS).await?;

        let snapshot = {
            let mut students_guard = students_arc.write().await;

            students_guard.reserve(payload.len());
            for s in payload {
                let id = s.id.clone();
                students_guard.insert(id, s);
            }
            students_guard.sort_unstable_by(|_, a, _, b| a.name.cmp(&b.name));
            
            students_guard.clone()
        };

        Store::upsert(snapshot).await
    }

    pub async fn batch_remove(
        workspace: impl Into<String>,
        ids: Option<Vec<String>>,
        nodes: Option<Vec<String>>,
    ) -> Result<()> {
        let students_arc = Store::find::<Students>(workspace, STUDENTS).await?;
        let ids = ids.map(|v| v.into_iter().collect::<HashSet<String>>());
        let nodes = nodes.map(|v| v.into_iter().collect::<HashSet<String>>());

        if ids.is_none() && nodes.is_none() {
            return Ok(());
        }

        let snapshot = {
            let mut students_guard = students_arc.write().await;

            students_guard.retain(|_, s| {
                let by_id = ids.as_ref().map_or(true, |set| set.contains(&s.id));
                let by_node = nodes.as_ref().map_or(true, |set| set.contains(&s.node));

                let should_delete = (ids.is_none() || by_id) && (nodes.is_none() || by_node);
                !should_delete
            });

            students_guard.clone()
        };

        Store::upsert(snapshot).await
    }
}