use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecipeInstruction {
    pub id: i32,
    pub recipe_id: i32,
    pub order_idx: i32,
    pub title: String,
}

pub struct CreateRecipeInstructionParams {
    pub recipe_id: i32,
    pub order_idx: i32,
    pub title: String,
}

impl CreateRecipeInstructionParams {
    pub fn new(
        recipe_id: i32,
        order_idx: i32,
        title: String,
    ) -> Self {
        Self {
            recipe_id,
            order_idx,
            title,
        }
    }
}

impl RecipeInstruction {
    pub async fn create(db: &PgPool, create_params: &CreateRecipeInstructionParams) -> Result<Option<Self>, crate::models::Error> {
        let recipe_instruction = sqlx::query_as(
            "INSERT INTO recipe_instructions (recipe_id, order_idx, title)
             VALUES ($1, $2, $3)
             RETURNING *"
        )
        .bind(create_params.recipe_id)
        .bind(create_params.order_idx)
        .bind(&create_params.title)
        .fetch_optional(db)
        .await?;

        Ok(recipe_instruction)
    }

    pub async fn find_by_id(db: &PgPool, id: i32) -> Result<Option<Self>, crate::models::Error> {
        let recipe_instruction = sqlx::query_as("SELECT * FROM recipe_instructions WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;

        Ok(recipe_instruction)
    }

    pub async fn update(&self, db: &PgPool) -> Result<(), crate::models::Error> {
        sqlx::query(
            "UPDATE recipe_instructions
             SET recipe_id = $1, order_idx = $2, title = $3
             WHERE id = $8"
        )
        .bind(self.recipe_id)
        .bind(self.order_idx)
        .bind(&self.title)
        .bind(self.id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn delete(db: &PgPool, id: i32) -> Result<(), crate::models::Error> {
        sqlx::query("DELETE FROM recipe_instructions WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn find_by_recipe_id_and_order_idx(db: &PgPool, recipe_id: i32, order_idx: i32) -> Result<Option<RecipeInstruction>, crate::models::Error> {
        let recipe_instruction = sqlx::query_as("SELECT * FROM recipe_instructions WHERE recipe_id = $1 AND order_idx = $2")
            .bind(recipe_id)
            .bind(order_idx)
            .fetch_optional(db)
            .await?;
        Ok(recipe_instruction)
    }
}

