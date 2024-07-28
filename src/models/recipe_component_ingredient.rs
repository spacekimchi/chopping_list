use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RecipeComponentIngredient {
    pub id: i32,
    pub recipe_component_id: i32,
    pub ingredient_id: i32,
    pub unit_id: i32,
    pub quantity_numerator: i32,
    pub quantity_denominator: i32,
    pub is_optional: bool,
}

pub struct CreateRecipeComponentIngredientParams {
    pub recipe_component_id: i32,
    pub ingredient_id: i32,
    pub unit_id: i32,
    pub quantity_numerator: i32,
    pub quantity_denominator: i32,
    pub is_optional: bool,
}

impl CreateRecipeComponentIngredientParams {
    pub fn new(
        recipe_component_id: i32,
        ingredient_id: i32,
        unit_id: i32,
        quantity_numerator: i32,
        quantity_denominator: i32,
    ) -> Self {
        Self {
            recipe_component_id,
            ingredient_id,
            unit_id,
            quantity_numerator,
            quantity_denominator,
            is_optional: false,
        }
    }

    pub fn with_optional(mut self, is_optional: bool) -> Self {
        self.is_optional = is_optional;
        self
    }
}

impl RecipeComponentIngredient {
    pub async fn create(db: &PgPool, create_params: &CreateRecipeComponentIngredientParams) -> Result<Option<Self>, crate::models::Error> {
        let recipe_component_ingredient = sqlx::query_as(
            "INSERT INTO recipe_component_ingredients (recipe_component_id, ingredient_id, unit_id, quantity_numerator, quantity_denominator, is_optional)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING *"
        )
        .bind(create_params.recipe_component_id)
        .bind(create_params.ingredient_id)
        .bind(create_params.unit_id)
        .bind(create_params.quantity_numerator)
        .bind(create_params.quantity_denominator)
        .bind(create_params.is_optional)
        .fetch_optional(db)
        .await?;

        Ok(recipe_component_ingredient)
    }

    pub async fn find_by_id(db: &PgPool, id: i32) -> Result<Option<Self>, crate::models::Error> {
        let recipe_component_ingredient = sqlx::query_as("SELECT * FROM recipe_component_ingredients WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;

        Ok(recipe_component_ingredient)
    }

    pub async fn update(&self, db: &PgPool) -> Result<(), crate::models::Error> {
        sqlx::query(
            "UPDATE recipe_component_ingredients
             SET recipe_component_id = $1, ingredient_id = $2, unit_id = $3,
                 quantity_numerator = $4, quantity_denominator = $5, is_optional = $6
             WHERE id = $8"
        )
        .bind(self.recipe_component_id)
        .bind(self.ingredient_id)
        .bind(self.unit_id)
        .bind(self.quantity_numerator)
        .bind(self.quantity_denominator)
        .bind(self.is_optional)
        .bind(self.id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn delete(db: &PgPool, id: i32) -> Result<(), crate::models::Error> {
        sqlx::query("DELETE FROM recipe_component_ingredients WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }
}

