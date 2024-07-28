use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecipeInstructionStep {
    pub id: i32,
    pub recipe_id: i32,
    pub recipe_instruction_id: i32,
    pub step_number: i32,
    pub content: String,
}

pub struct CreateRecipeInstructionStepParams {
    pub recipe_id: i32,
    pub recipe_instruction_id: i32,
    pub step_number: i32,
    pub content: String,
}

impl CreateRecipeInstructionStepParams {
    pub fn new(
        recipe_id: i32,
        recipe_instruction_id: i32,
        step_number: i32,
        content: String,
    ) -> Self {
        Self {
            recipe_id,
            recipe_instruction_id,
            step_number,
            content,
        }
    }
}

impl RecipeInstructionStep {
    pub async fn create(db: &PgPool, create_params: &CreateRecipeInstructionStepParams) -> Result<Option<Self>, crate::models::Error> {
        let recipe_instruction_step = sqlx::query_as(
            "INSERT INTO recipe_instruction_steps (recipe_id, recipe_instruction_id, step_number, content)
             VALUES ($1, $2, $3, $4)
             RETURNING *"
        )
        .bind(create_params.recipe_id)
        .bind(create_params.recipe_instruction_id)
        .bind(create_params.step_number)
        .bind(&create_params.content)
        .fetch_optional(db)
        .await?;

        Ok(recipe_instruction_step)
    }

    pub async fn find_by_id(db: &PgPool, id: i32) -> Result<Option<Self>, crate::models::Error> {
        let recipe_instruction_step = sqlx::query_as("SELECT * FROM recipe_instruction_steps WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;

        Ok(recipe_instruction_step)
    }

    pub async fn update(&self, db: &PgPool) -> Result<(), crate::models::Error> {
        sqlx::query(
            "UPDATE recipe_instruction_steps
             SET recipe_id = $1, ingredient_id = $2, unit_id = $3,
                 quantity_numerator = $4, quantity_denominator = $5, is_optional = $6
             WHERE id = $8"
        )
        .bind(self.recipe_id)
        .bind(self.recipe_instruction_id)
        .bind(self.step_number)
        .bind(&self.content)
        .bind(self.id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn delete(db: &PgPool, id: i32) -> Result<(), crate::models::Error> {
        sqlx::query("DELETE FROM recipe_instruction_steps WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn next_step_number(db: &PgPool, recipe_instruction_id: i32) -> Result<i32, crate::models::Error> {
        let max_step_number: Option<i32> = sqlx::query_scalar::<_, Option<i32>>(
            "SELECT MAX(step_number) FROM recipe_instruction_steps WHERE recipe_instruction_id = $1"
        )
            .bind(recipe_instruction_id)
            .fetch_one(db)
            .await?;

        Ok(max_step_number.unwrap_or(0) + 1)
    }
}

