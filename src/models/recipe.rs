use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct Recipe {
    pub id: i64,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub hidden: Option<bool>,
    pub prep_time: Option<i64>,
    pub cook_time: Option<i64>,
    pub servings: Option<i64>,
}

pub struct CreateParams {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub hidden: Option<bool>,
    pub prep_time: Option<i64>,
    pub cook_time: Option<i64>,
    pub rest_time: Option<i64>,
    pub servings: Option<i64>,
}

impl CreateParams {
    pub fn new(user_id: uuid::Uuid, name: String) -> Self {
        Self {
            user_id,
            name,
            description: None,
            hidden: None,
            prep_time: None,
            cook_time: None,
            rest_time: None,
            servings: None,
        }
    }

    pub fn with_hidden(mut self) -> Self {
        self.hidden = Some(true);
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn with_cook_time(mut self, cook_time: i64) -> Self {
        self.cook_time = Some(cook_time);
        self
    }

    pub fn with_prep_time(mut self, prep_time: i64) -> Self {
        self.prep_time = Some(prep_time);
        self
    }

    pub fn with_rest_time(mut self, rest_time: i64) -> Self {
        self.rest_time = Some(rest_time);
        self
    }

    pub fn with_servings(mut self, servings: i64) -> Self {
        self.servings = Some(servings);
        self
    }
}

impl Recipe {
    pub async fn create_recipe(db: &PgPool, create_params: &CreateParams) -> Result<Option<Self>, crate::models::Error> {
        // Just bind everything. If it is None, it will convert to NULL
        let recipe = sqlx::query_as("INSERT INTO recipes (user_id, name, description, hidden, prep_time, cook_time, rest_time, servings) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *")
            .bind(create_params.user_id)
            .bind(create_params.name.clone())
            .bind(create_params.description.clone())
            .bind(create_params.hidden)
            .bind(create_params.prep_time)
            .bind(create_params.cook_time)
            .bind(create_params.rest_time)
            .bind(create_params.servings)
            .fetch_optional(db)
            .await?;
        Ok(recipe)
    }
}

