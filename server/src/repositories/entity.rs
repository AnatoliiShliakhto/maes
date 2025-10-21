use crate::services::*;
use ::shared::{common::*, models::*};
use ::std::collections::HashSet;

pub struct EntityRepository;

impl EntityRepository {
    pub async fn init(workspace: Entity) -> Result<()> {
        if !Store::get_path(&workspace.id, ENTITIES).exists() {
            let entities = Entities::new(workspace);
            Store::upsert(entities).await?;
        };
        Ok(())
    }

    pub async fn list_by_filter(
        workspace: impl Into<String>,
        kind: Option<Vec<EntityKind>>,
        ids: Option<Vec<String>>,
        nodes: Option<Vec<String>>,
    ) -> Result<Vec<Entity>> {
        let entities_arc = Store::find::<Entities>(workspace, ENTITIES).await?;

        let kinds = kind.map(|vec| vec.into_iter().collect::<HashSet<EntityKind>>());
        let ids = ids.map(|vec| vec.into_iter().collect::<HashSet<String>>());
        let nodes = nodes.map(|vec| vec.into_iter().collect::<HashSet<String>>());

        let mut entities = {
            let entities_guard = entities_arc.read().await;
            let mut out = Vec::with_capacity(entities_guard.len());
            for e in entities_guard.values() {
                if kinds.as_ref().map_or(true, |set| set.contains(&e.kind))
                    && ids.as_ref().map_or(true, |set| set.contains(&e.id))
                    && nodes.as_ref().map_or(true, |set| set.contains(&e.node))
                {
                    out.push(e.clone());
                }
            }
            out
        };
        entities.sort_unstable_by(|a, b| a.path.cmp(&b.path).then(a.name.cmp(&b.name)));

        Ok(entities)
    }

    pub async fn find(workspace: impl Into<String>, id: impl AsRef<str>) -> Result<Entity> {
        let entities_arc = Store::find::<Entities>(workspace, ENTITIES).await?;
        let entity = {
            let entities_guard = entities_arc.read().await;
            entities_guard.get(id.as_ref()).cloned()
        };
        entity.ok_or_else(|| (StatusCode::NOT_FOUND, "entity-not-found").into())
    }

    pub async fn upsert(workspace: impl Into<String>, entity: Entity) -> Result<()> {
        let entities_arc = Store::find::<Entities>(workspace, ENTITIES).await?;

        let snapshot = {
            let mut entities_guard = entities_arc.write().await;
            entities_guard.insert(entity.id.clone(), entity);

            entities_guard.clone()
        };

        Store::upsert(snapshot).await
    }

    pub async fn delete(
        workspace: impl Into<String>,
        id: Option<String>,
        node: Option<String>,
    ) -> Result<()> {
        let entities_arc = Store::find::<Entities>(workspace, ENTITIES).await?;

        let to_delete = {
            let entities_guard = entities_arc.read().await;
            if entities_guard.is_empty() {
                Vec::new()
            } else {
                let mut out = Vec::with_capacity(entities_guard.len());
                for e in entities_guard.values() {
                    if id.as_ref().map_or(true, |k| e.id == *k)
                        && node.as_ref().map_or(true, |n| e.node == *n)
                    {
                        out.push(e.id.clone());
                    }
                }
                out
            }
        };

        if to_delete.is_empty() {
            return Ok(());
        }

        let snapshot = {
            let mut entities_guard = entities_arc.write().await;
            for k in &to_delete {
                entities_guard.remove(k);
            }
            entities_guard.clone()
        };

        Store::upsert(snapshot).await
    }

    pub async fn batch_remove(
        workspace: impl Into<String>,
        ids: Option<Vec<String>>,
        nodes: Option<Vec<String>>,
    ) -> Result<()> {
        let ws_id = workspace.into();
        let entities_arc = Store::find::<Entities>(&ws_id, ENTITIES).await?;
        let ids = ids.map(|vec| vec.into_iter().collect::<HashSet<String>>());
        let nodes = nodes.map(|vec| vec.into_iter().collect::<HashSet<String>>());

        let to_delete = {
            let entities_guard = entities_arc.read().await;
            if entities_guard.is_empty() {
                Vec::new()
            } else {
                let mut out = Vec::with_capacity(entities_guard.len());
                for e in entities_guard.values() {
                    if ids.as_ref().map_or(true, |set| set.contains(&e.id))
                        && nodes.as_ref().map_or(true, |set| set.contains(&e.node))
                    {
                        out.push(e.id.clone());
                    }
                }
                out
            }
        };

        if to_delete.is_empty() {
            return Ok(());
        }

        let snapshot = {
            let mut entities_guard = entities_arc.write().await;
            for k in &to_delete {
                entities_guard.remove(k);
            }
            entities_guard.clone()
        };

        Store::upsert(snapshot).await?;
        Store::batch_remove(ws_id, to_delete).await
    }
}
