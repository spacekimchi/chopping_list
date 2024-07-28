pub mod ingredient;
pub mod recipe;
pub mod recipe_component;
pub mod recipe_component_ingredient;
pub mod tag;
pub mod user;
pub mod user_role;
pub mod unit;
pub mod recipe_instruction;
pub mod recipe_instruction_step;
pub mod recipe_tag;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error("Resource not found")]
    NotFound,
}
