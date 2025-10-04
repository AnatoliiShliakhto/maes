use ::shared::{common::*, models::*};
use ::surrealdb::{Surreal, engine::any::Any};

pub trait StudentsRepository {
}

impl StudentsRepository for Surreal<Any> {
}