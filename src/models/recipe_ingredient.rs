use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecipeIngredient {
    pub id: i64,
    pub recipe_id: i64,
    pub ingredient_id: i64,
    pub quantity: i64,
    pub unit: Option<String>,
    pub optional: bool,
}

pub struct CreateParams {
    pub recipe_id: i64,
    pub ingredient_id: i64,
    pub quantity: i64,
    pub unit: Option<String>,
    pub optional: bool,
}

impl CreateParams {
    pub fn new(
        recipe_id: i64,
        ingredient_id: i64,
        quantity: i64,
        unit: Option<String>,
        optional: bool
    ) -> Self {
        Self {
            recipe_id,
            ingredient_id,
            quantity,
            unit,
            optional,
        }
    }

    pub fn with_unit(mut self, unit: String) -> Self {
        self.unit = Some(unit);
        self
    }
}

impl RecipeIngredient {
    pub async fn create(db: &PgPool, create_params: &CreateParams) -> Result<Option<Self>, crate::models::Error> {
        // Just bind everything. If it is None, it will convert to NULL
        let recipe_ingredient = sqlx::query_as("INSERT INTO recipes (recipe_id, ingredient_id, quantity, unit, optional) VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(create_params.recipe_id)
            .bind(create_params.ingredient_id)
            .bind(create_params.quantity)
            .bind(create_params.unit.clone())
            .bind(create_params.optional)
            .fetch_optional(db)
            .await?;
        Ok(recipe_ingredient)
    }
}

