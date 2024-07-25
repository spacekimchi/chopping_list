pub mod ingredient;
pub mod instruction;
pub mod recipe;
pub mod recipe_component;
pub mod recipe_ingredient;
pub mod tag;
pub mod user;
pub mod user_role;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
