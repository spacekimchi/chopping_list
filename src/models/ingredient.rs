use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub struct CreateIngredientParams {
    pub name: String,
    pub description: String,
}

impl CreateIngredientParams {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
        }
    }
}

impl Ingredient {
    pub async fn create(db: &PgPool, params: &CreateIngredientParams) -> Result<Self, crate::models::Error> {
        let ingredient = sqlx::query_as(
            "INSERT INTO ingredients (name, description) VALUES ($1, $2) RETURNING *"
        )
        .bind(&params.name)
        .bind(&params.description)
        .fetch_one(db)
        .await?;

        Ok(ingredient)
    }

    pub async fn find_by_id(db: &PgPool, id: i32) -> Result<Option<Self>, crate::models::Error> {
        let ingredient = sqlx::query_as(
            "SELECT * FROM ingredients WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(db)
        .await?;

        Ok(ingredient)
    }

    pub async fn find_by_name(db: &PgPool, name: &str) -> Result<Option<Self>, crate::models::Error> {
        let ingredient = sqlx::query_as(
            "SELECT * FROM ingredients WHERE name = $1"
        )
        .bind(name)
        .fetch_optional(db)
        .await?;

        Ok(ingredient)
    }

    pub async fn update(&self, db: &PgPool) -> Result<(), crate::models::Error> {
        sqlx::query(
            "UPDATE ingredients SET name = $1, description = $2 WHERE id = $3"
        )
        .bind(&self.name)
        .bind(&self.description)
        .bind(self.id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn delete(&self, db: &PgPool) -> Result<(), crate::models::Error> {
        sqlx::query("DELETE FROM ingredients WHERE id = $1")
            .bind(self.id)
            .execute(db)
            .await?;

        Ok(())
    }

    pub async fn get_ingredients_by_recipe_id(db: &PgPool, recipe_id: i32) -> Result<Vec<Self>, crate::models::Error> {
        let ingredients = sqlx::query_as(
            "SELECT * FROM ingredients WHERE recipe_id = $1 ORDER BY name"
        )
        .bind(recipe_id)
        .fetch_all(db)
        .await?;

        Ok(ingredients)
    }

    pub async fn list(db: &PgPool, limit: i32, offset: i32) -> Result<Vec<Self>, crate::models::Error> {
        let ingredients = sqlx::query_as(
            "SELECT * FROM ingredients ORDER BY name LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(db)
        .await?;

        Ok(ingredients)
    }

    pub async fn search(db: &PgPool, query: &str, limit: i32) -> Result<Vec<Self>, crate::models::Error> {
        let ingredients = sqlx::query_as(
            "SELECT * FROM ingredients WHERE name ILIKE $1 ORDER BY name LIMIT $2"
        )
        .bind(format!("%{}%", query))
        .bind(limit)
        .fetch_all(db)
        .await?;

        Ok(ingredients)
    }

    async fn add_new_ingredient(db: &PgPool, name: String, description: Option<String>) -> Result<Ingredient, crate::models::Error> {
        let params = CreateIngredientParams::new(name, description.unwrap_or_default());

        Ingredient::create(db, &params).await
    }

    async fn get_ingredient_by_name(db: &PgPool, name: &str) -> Result<Option<Ingredient>, crate::models::Error> {
        Ingredient::find_by_name(db, name).await
    }

    async fn update_ingredient_description(&self, db: &PgPool, new_description: String) -> Result<(), crate::models::Error> {
        if let Some(mut ingredient) = Ingredient::find_by_id(db, self.id).await? {
            ingredient.description = new_description;
            ingredient.update(db).await
        } else {
            Err(crate::models::Error::NotFound)
        }
    }
}
