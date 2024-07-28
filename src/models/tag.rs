use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

pub struct CreateTagParams {
    pub name: String,
}


impl CreateTagParams {
    pub fn new(name: String) -> Self {
        Self {
            name,
        }
    }
}

impl Tag {
    pub async fn create(db: &PgPool, params: &CreateTagParams) -> Result<Option<Self>, crate::models::Error> {
        let tag = sqlx::query_as(
            "INSERT INTO tags (name) 
             VALUES ($1) 
             RETURNING *"
        )
        .bind(&params.name)
        .fetch_optional(db)
        .await?;

        Ok(tag)
    }


    pub async fn find_by_id(db: &PgPool, id: i32) -> Result<Option<Self>, crate::models::Error> {
        let recipe_ingredient = sqlx::query_as("SELECT * FROM tags WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;

        Ok(recipe_ingredient)
    }

    pub async fn find_by_name(db: &PgPool, name: &str) -> Result<Option<Self>, crate::models::Error> {
        let unit = sqlx::query_as("SELECT * FROM tags WHERE name = $1")
            .bind(name)
            .fetch_optional(db)
            .await?;

        Ok(unit)
    }

    pub async fn find_by_recipe_id(db: &PgPool, recipe_id: i32) -> Result<Vec<Self>, crate::models::Error> {
        let tags = sqlx::query_as(
            "SELECT t.* FROM tags t
             JOIN recipes_tags rt ON t.id = rt.tag_id
             WHERE rt.recipe_id = $1"
        )
        .bind(recipe_id)
        .fetch_all(db)
        .await?;

        Ok(tags)
    }

    pub async fn update(&self, db: &PgPool) -> Result<(), crate::models::Error> {
        sqlx::query(
            "UPDATE tags
             SET name = $1
             WHERE id = $2"
        )
        .bind(self.name.clone())
        .bind(self.id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn delete(db: &PgPool, id: i32) -> Result<(), crate::models::Error> {
        sqlx::query("DELETE FROM tags WHERE id = $1")
            .bind(id)
            .execute(db)
            .await?;

        Ok(())
    }
}

