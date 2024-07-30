use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

use crate::models::tag::{Tag, CreateTagParams};
use crate::models::ingredient::Ingredient;
use crate::models::recipe_tag::{RecipeTag, CreateRecipeTagParams};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Recipe {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub prep_time: Option<i32>,
    pub cook_time: Option<i32>,
    pub rest_time: Option<i32>,
    pub servings: Option<i32>,
    pub source_url: Option<String>,
}

pub struct CreateRecipeParams {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub prep_time: Option<i32>,
    pub cook_time: Option<i32>,
    pub rest_time: Option<i32>,
    pub servings: Option<i32>,
    pub source_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullRecipeDetails {
    pub recipe_id: i32,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub prep_time: Option<i32>,
    pub cook_time: Option<i32>,
    pub rest_time: Option<i32>,
    pub servings: Option<i32>,
    pub source_url: Option<String>,
    pub recipe_components: Vec<FullRecipeComponent>,
    pub recipe_instructions: Vec<FullRecipeInstruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullRecipeComponent {
    pub name: String,
    pub is_optional: bool,
    pub component_ingredients: Vec<FullRecipeComponentIngredient>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullRecipeComponentIngredient {
    pub ingredient_id: i32,
    pub unit: String,
    pub quantity_numerator: i32,
    pub quantity_denominator: i32,
    pub is_optional: bool,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullRecipeInstruction {
    pub order_idx: i32,
    pub title: String,
    pub instruction_steps: Vec<FullRecipeInstructionStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullRecipeInstructionStep {
    pub step_number: i32,
    pub content: String,
}

impl CreateRecipeParams {
    pub fn new(user_id: uuid::Uuid, name: String, description: String) -> Self {
        Self {
            user_id,
            name,
            description,
            is_public: false,
            prep_time: None,
            cook_time: None,
            rest_time: None,
            servings: None,
            source_url: None,
        }
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

    pub async fn get_full_recipe_details(db: &PgPool, user_id: &uuid::Uuid, recipe_id: i32) -> Result<FullRecipeDetails, crate::models::Error> {
        // Get basic recipe data
        let recipe = sqlx::query!(
            r#"
        SELECT id as recipe_id, name, description, is_public, prep_time, cook_time, rest_time, servings, source_url
        FROM recipes
        WHERE id = $1 AND user_id = $2
        "#,
        recipe_id,
        user_id
        )
            .fetch_one(db)
            .await?;

        // Get components and their ingredients
        let components = sqlx::query!(
            r#"
        SELECT
            rc.name as component_name,
            rc.is_optional as component_is_optional,
            json_agg(json_build_object(
                'ingredient_id', i.id,
                'unit', u.name,
                'quantity_numerator', rci.quantity_numerator,
                'quantity_denominator', rci.quantity_denominator,
                'is_optional', rci.is_optional,
                'name', i.name,
                'description', i.description
            ) ORDER BY rci.id) as component_ingredients
        FROM recipe_components rc
        LEFT JOIN recipe_component_ingredients rci ON rc.id = rci.recipe_component_id
        LEFT JOIN ingredients i ON rci.ingredient_id = i.id
        LEFT JOIN units u ON rci.unit_id = u.id
        WHERE rc.recipe_id = $1
        GROUP BY rc.id
        ORDER BY rc.id
        "#,
        recipe_id
            )
            .fetch_all(db)
            .await?;

        // Get instructions and their steps
        let instructions = sqlx::query!(
            r#"
        SELECT
            ri.order_idx,
            ri.title,
            json_agg(json_build_object(
                'step_number', ris.step_number,
                'content', ris.content
            ) ORDER BY ris.step_number) as instruction_steps
        FROM recipe_instructions ri
        LEFT JOIN recipe_instruction_steps ris ON ri.id = ris.recipe_instruction_id
        WHERE ri.recipe_id = $1
        GROUP BY ri.id
        ORDER BY ri.order_idx
        "#,
        recipe_id
        )
            .fetch_all(db)
            .await?;

        // Convert the raw data into the FullRecipeDetails struct
        let recipe_components: Vec<FullRecipeComponent> = components
            .into_iter()
            .map(|c| FullRecipeComponent {
                name: c.component_name,
                is_optional: c.component_is_optional,
                component_ingredients: serde_json::from_value(c.component_ingredients.unwrap_or_default()).unwrap_or_default(),
            })
        .collect();

        let recipe_instructions: Vec<FullRecipeInstruction> = instructions
            .into_iter()
            .map(|i| FullRecipeInstruction {
                order_idx: i.order_idx,
                title: i.title,
                instruction_steps: serde_json::from_value(i.instruction_steps.unwrap_or_default()).unwrap_or_default(),
            })
        .collect();

        Ok(FullRecipeDetails {
            recipe_id: recipe.recipe_id,
            name: recipe.name,
            description: recipe.description,
            is_public: recipe.is_public,
            prep_time: recipe.prep_time,
            cook_time: recipe.cook_time,
            rest_time: recipe.rest_time,
            servings: recipe.servings,
            source_url: recipe.source_url,
            recipe_components,
            recipe_instructions,
        })
    }
}

