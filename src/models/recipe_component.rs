use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecipeComponent {
    pub id: i32,
    pub recipe_id: i32,
    pub name: String,
    pub is_optional: bool,
}

pub struct CreateRecipeComponentParams {
    pub recipe_id: i32,
    pub name: String,
    pub is_optional: bool,
}

impl CreateRecipeComponentParams {
    pub fn new(
        recipe_id: i32,
        name: String,
        is_optional: bool,
    ) -> Self {
        Self {
            recipe_id,
            name,
            is_optional
        }
    }
}

impl RecipeComponent {
    pub async fn create(db: &PgPool, create_params: &CreateRecipeComponentParams) -> Result<Option<Self>, crate::models::Error> {
        let recipe_component = sqlx::query_as(
            "INSERT INTO recipe_components (recipe_id, name, is_optional)
             VALUES ($1, $2, $3)
             RETURNING *"
        )
        .bind(create_params.recipe_id)
        .bind(&create_params.name)
        .bind(create_params.is_optional)
        .fetch_optional(db)
        .await?;

        Ok(recipe_component)
    }

    pub async fn find_by_id(db: &PgPool, id: i32) -> Result<Option<Self>, crate::models::Error> {
        let recipe_component = sqlx::query_as("SELECT * FROM recipe_components WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;

        Ok(recipe_component)
    }

    pub async fn find_by_recipe_id_and_name(db: &PgPool, recipe_id: i32, name: &String) -> Result<Option<RecipeComponent>, crate::models::Error> {
        let recipe_component = sqlx::query_as("SELECT * FROM recipe_components WHERE recipe_id = $1 AND name = $2")
            .bind(recipe_id)
            .bind(&name)
            .fetch_optional(db)
            .await?;

        Ok(recipe_component)
    }

    pub async fn update(&self, db: &PgPool) -> Result<(), crate::models::Error> {
        sqlx::query(
            "UPDATE recipe_components
             SET recipe_id = $1, name = $2, is_optional = $3
             WHERE id = $4"
        )
        .bind(self.recipe_id)
        .bind(&self.name)
        .bind(self.is_optional)
        .bind(self.id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn delete(db: &PgPool, id: i32) -> Result<(), crate::models::Error> {
        sqlx::query("DELETE FROM recipe_components WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }
}

