use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecipeTag {
    pub id: i32,
    pub recipe_id: i32,
    pub tag_id: i32,
}

pub struct CreateRecipeTagParams {
    pub recipe_id: i32,
    pub tag_id: i32,
}

impl RecipeTag {
    pub async fn create(db: &PgPool, params: &CreateRecipeTagParams) -> Result<Self, crate::models::Error> {
        let recipe_tag = sqlx::query_as(
            "INSERT INTO recipes_tags (recipe_id, tag_id) VALUES ($1, $2) RETURNING *"
        )
        .bind(params.recipe_id)
        .bind(params.tag_id)
        .fetch_one(db)
        .await?;

        Ok(recipe_tag)
    }

    pub async fn find_by_recipe_id(db: &PgPool, recipe_id: i32) -> Result<Vec<Self>, crate::models::Error> {
        let recipe_tags = sqlx::query_as(
            "SELECT * FROM recipes_tags WHERE recipe_id = $1"
        )
        .bind(recipe_id)
        .fetch_all(db)
        .await?;

        Ok(recipe_tags)
    }

    pub async fn find_by_tag_id(db: &PgPool, tag_id: i32) -> Result<Vec<Self>, crate::models::Error> {
        let recipe_tags = sqlx::query_as(
            "SELECT * FROM recipes_tags WHERE tag_id = $1"
        )
        .bind(tag_id)
        .fetch_all(db)
        .await?;

        Ok(recipe_tags)
    }

    pub async fn delete(db: &PgPool, recipe_id: i32, tag_id: i32) -> Result<(), crate::models::Error> {
        sqlx::query(
            "DELETE FROM recipes_tags WHERE recipe_id = $1 AND tag_id = $2"
        )
        .bind(recipe_id)
        .bind(tag_id)
        .execute(db)
        .await?;

        Ok(())
    }
}
