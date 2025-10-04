use crate::services::*;
use ::shared::{common::*, models::*};
use ::std::collections::HashSet;

pub struct TaskRepository;

impl TaskRepository {
    pub async fn init(workspace: impl AsRef<str>) -> Result<()> {
        let ws_id = workspace.as_ref();
        if let Ok(path) = Store::get_path(ws_id, TASKS)
            && !path.exists()
        {
            let tasks = Tasks::new(ws_id);
            Store::upsert(tasks).await?;
        };
        Ok(())
    }

    pub async fn list_by_filter(
        workspace: impl Into<String>,
        kind: Option<Vec<EntityKind>>,
        nodes: Option<Vec<String>>,
    ) -> Result<Vec<Task>> {
        let tasks_arc = Store::find::<Tasks>(workspace, TASKS).await?;
        let kinds = kind.map(|vec| vec.into_iter().collect::<HashSet<EntityKind>>());
        let nodes = nodes.map(|vec| vec.into_iter().collect::<HashSet<String>>());

        let mut tasks = {
            let tasks_guard = tasks_arc.read().await;
            let mut out = Vec::with_capacity(tasks_guard.len());
            for t in tasks_guard.values() {
                if kinds.as_ref().map_or(true, |set| set.contains(&t.kind))
                    && nodes.as_ref().map_or(true, |set| set.contains(&t.node))
                {
                    out.push(t.clone());
                }
            }
            out
        };

        tasks.sort_unstable_by(|a, b| a.name.cmp(&b.name).then(a.path.cmp(&b.path)));
        Ok(tasks)
    }

    pub async fn upsert(workspace: impl Into<String>, task: Task) -> Result<()> {
        let tasks_arc = Store::find::<Tasks>(workspace, TASKS).await?;
        let snapshot = {
            let mut tasks_guard = tasks_arc.write().await;
            let id = task.id.clone();
            tasks_guard.insert(id, task);
            tasks_guard.clone()
        };
        Store::upsert(snapshot).await
    }

    pub async fn delete(
        workspace: impl Into<String>,
        id: Option<String>,
        node: Option<String>,
    ) -> Result<()> {
        let tasks_arc = Store::find::<Tasks>(workspace, TASKS).await?;

        let to_delete = {
            let tasks_guard = tasks_arc.read().await;
            if tasks_guard.is_empty() {
                Vec::new()
            } else {
                let mut out = Vec::with_capacity(tasks_guard.len());
                for t in tasks_guard.values() {
                    if id.as_ref().map_or(true, |k| t.id == *k)
                        && node.as_ref().map_or(true, |n| t.node == *n)
                    {
                        out.push(t.id.clone());
                    }
                }
                out
            }
        };

        if to_delete.is_empty() {
            return Ok(());
        }

        let snapshot = {
            let mut tasks_guard = tasks_arc.write().await;
            for k in &to_delete {
                tasks_guard.remove(k);
            }
            tasks_guard.clone()
        };

        Store::upsert(snapshot).await
    }
}