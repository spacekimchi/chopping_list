use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

use crate::models::tag::{Tag, CreateTagParams};
use crate::models::ingredient::{Ingredient, CreateIngredientParams};
use crate::models::recipe_tag::{RecipeTag, CreateRecipeTagParams};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Recipe {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub prep_time: Option<i32>,
    pub cook_time: Option<i32>,
    pub servings: Option<i32>,
    pub source_url: Option<String>,
}

pub struct CreateRecipeParams {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub prep_time: Option<i32>,
    pub cook_time: Option<i32>,
    pub rest_time: Option<i32>,
    pub servings: Option<i32>,
    pub source_url: Option<String>,
}

impl CreateRecipeParams {
    pub fn new(user_id: uuid::Uuid, name: String) -> Self {
        Self {
            user_id,
            name,
            description: None,
            is_public: false,
            prep_time: None,
            cook_time: None,
            rest_time: None,
            servings: None,
            source_url: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_cook_time(mut self, cook_time: i32) -> Self {
        self.cook_time = Some(cook_time);
        self
    }

    pub fn with_prep_time(mut self, prep_time: i32) -> Self {
        self.prep_time = Some(prep_time);
        self
    }

    pub fn with_rest_time(mut self, rest_time: i32) -> Self {
        self.rest_time = Some(rest_time);
        self
    }

    pub fn with_servings(mut self, servings: i32) -> Self {
        self.servings = Some(servings);
        self
    }

    pub fn with_source_url(mut self, source_url: String) -> Self {
        self.source_url = Some(source_url);
        self
    }
}

impl Recipe {
    pub async fn create(db: &PgPool, create_params: &CreateRecipeParams) -> Result<Option<Self>, crate::models::Error> {
        // Just bind everything. If it is None, it will convert to NULL
        let recipe = sqlx::query_as(
            "INSERT INTO recipes (user_id, name, description, is_public, prep_time, cook_time, rest_time, servings, source_url)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING *"
             )
            .bind(create_params.user_id)
            .bind(&create_params.name)
            .bind(&create_params.description)
            .bind(create_params.is_public)
            .bind(create_params.prep_time)
            .bind(create_params.cook_time)
            .bind(create_params.rest_time)
            .bind(create_params.servings)
            .bind(&create_params.source_url)
            .fetch_optional(db)
            .await?;
        Ok(recipe)
    }

    pub async fn add_tag(&self, db: &PgPool, tag_name: &str) -> Result<(), crate::models::Error> {
        let tag = match Tag::find_by_name(db, tag_name).await? {
            Some(t) => t,
            None => {
                let tag_params = CreateTagParams { name: tag_name.to_string() };
                Tag::create(db, &tag_params).await?.expect("Failed to create tag")
            }
        };

        let recipe_tag_params = CreateRecipeTagParams {
            recipe_id: self.id,
            tag_id: tag.id,
        };

        RecipeTag::create(db, &recipe_tag_params).await?;

        Ok(())
    }

    pub async fn get_ingredients(&self, db: &PgPool) -> Result<Vec<Ingredient>, crate::models::Error> {
        Ok(Ingredient::get_ingredients_by_recipe_id(db, self.id).await?)
    }

    pub async fn get_tags(&self, db: &PgPool) -> Result<Vec<Tag>, crate::models::Error> {
        Ok(Tag::find_by_recipe_id(db, self.id).await?)
    }
}

