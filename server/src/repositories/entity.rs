use ::shared::{common::*, models::*};
use ::surrealdb::{Surreal, engine::any::Any};

pub trait EntityRepository {
    async fn list_entities(&self, kind: EntityKind) -> Result<Vec<Entity>>;
    async fn find_entity(&self, id: impl Into<String>) -> Result<Entity>;
    async fn upsert_entity(&self, entity: Entity) -> Result<()>;
    async fn delete_entity(&self, id: impl Into<String>) -> Result<()>;
}

impl EntityRepository for Surreal<Any> {
    async fn list_entities(&self, kind: EntityKind) -> Result<Vec<Entity>> {
        let sql = include_str!("../../res/surql/entity/list.surql");
        self.query(sql)
            .bind(("kind", kind))
            .await?
            .take::<Vec<Entity>>(0)
            .err_into()
    }

    async fn find_entity(&self, id: impl Into<String>) -> Result<Entity> {
        let sql = include_str!("../../res/surql/entity/get_by_id.surql");
        self.query(sql)
            .bind(("id", id.into()))
            .await?
            .take::<Option<Entity>>(0)?
            .ok_or_else(|| (StatusCode::NOT_FOUND, "entity-not-found").into())
    }

    async fn upsert_entity(&self, entity: Entity) -> Result<()> {
        let sql = include_str!("../../res/surql/entity/upsert.surql");
        self.query(sql).bind(entity).await.discard()
    }

    async fn delete_entity(&self, id: impl Into<String>) -> Result<()> {
        let sql = include_str!("../../res/surql/entity/delete.surql");
        self.query(sql).bind(("id", id.into())).await.discard()
    }
}