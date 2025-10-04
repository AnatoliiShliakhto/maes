use ::std::sync::LazyLock;
use ::surrealdb::{Surreal, engine::any::Any};

pub static DB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);