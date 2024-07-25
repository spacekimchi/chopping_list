use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Ingredient {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

pub struct CreateParams {
    pub name: String,
    pub description: Option<String>,
}

impl CreateParams {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}

impl Ingredient {
    pub async fn create(db: &PgPool, create_params: &CreateParams) -> Result<Option<Self>, crate::models::Error> {
        // Just bind everything. If it is None, it will convert to NULL
        let ingredient = sqlx::query_as("INSERT INTO ingredients (name, description) VALUES ($1, $2) RETURNING *")
            .bind(create_params.name.clone())
            .bind(create_params.description.clone())
            .fetch_optional(db)
            .await?;
        Ok(ingredient)
    }
}

