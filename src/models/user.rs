use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};
use crate::models::recipe::Recipe;
use crate::models::recipe_component::RecipeComponent;

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, FromRow)]
pub struct CreateUserParams {
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

impl CreateUserParams {
    pub fn new(email: String, username: String, password_hash: String) -> Self {
        Self {
            email,
            username,
            password_hash,
        }
    }

    pub fn new_with_default_password(email: String, username: String) -> Self {
        Self {
            email,
            username,
            password_hash: String::from(r"$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw"),
        }
    }

    pub fn with_default_password(mut self) -> Self {
        // hunter42
        self.password_hash = String::from(r"$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw");
        self
    }
}

impl User {
    pub async fn create_user(db: &PgPool, create_user_params: &CreateUserParams) -> Result<Option<Self>, crate::models::Error> {
        let user = sqlx::query_as("INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3) RETURNING *")
            .bind(&create_user_params.email)
            .bind(&create_user_params.username)
            .bind(&create_user_params.password_hash)
            .fetch_optional(db)
            .await?;
        Ok(user)
    }

    pub async fn find_by_email(db: &PgPool, email: &String) -> Result<User, crate::models::Error> {
        let user = sqlx::query_as("SELECT id, username, email, password_hash FROM users WHERE email = $1")
            .bind(email.clone())
            .fetch_one(db)
            .await?;

        Ok(user)
    }

    pub async fn find_by_api_key(db: &PgPool, api_key: &str) -> Result<Option<User>, crate::models::Error> {
        let user = sqlx::query_as("SELECT id, username, email, password_hash, api_key FROM users WHERE api_key = $1")
            .bind(api_key)
            .fetch_optional(db)
            .await?;

        Ok(user)
    }

    pub async fn generate_api_key(&mut self, db: &PgPool) -> Result<String, crate::models::Error> {
        let api_key = uuid::Uuid::new_v4().to_string();
        sqlx::query("UPDATE users SET api_key = $1 WHERE id = $2")
            .bind(&api_key)
            .bind(self.id)
            .execute(db)
            .await?;

        Ok(api_key)
    }

    pub async fn update(&self, db: &PgPool) -> Result<(), crate::models::Error> {
        sqlx::query(
            "UPDATE users
             SET email = $1, username = $2, password_hash = $3
             WHERE id = $4"
        )
        .bind(&self.email)
        .bind(&self.username)
        .bind(&self.password_hash)
        .bind(self.id)
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn get_recipes(&self, db: &PgPool) -> Result<Vec<Recipe>, crate::models::Error> {
        let recipes = sqlx::query_as("SELECT * FROM recipes WHERE user_id = $1")
            .bind(self.id)
            .fetch_all(db)
            .await?;
        Ok(recipes)
    }

    pub async fn get_recipe_by_id(&self, db: &PgPool, recipe_id: i32) -> Result<Recipe, crate::models::Error> {
        let recipe = sqlx::query_as("SELECT * FROM recipes WHERE user_id = $1 AND id = $2 LIMIT 1")
            .bind(self.id)
            .bind(recipe_id)
            .fetch_one(db)
            .await?;
        Ok(recipe)
    }

    pub async fn get_recipe_components_by_recipe_id(&self, db: &PgPool, recipe_id: i32) -> Result<Vec<RecipeComponent>, crate::models::Error> {
        let recipe_components = sqlx::query_as("SELECT * FROM recipe_components WHERE user_id = $1 AND recipe_id = $2")
            .bind(self.id)
            .bind(recipe_id)
            .fetch_all(db)
            .await?;
        Ok(recipe_components)
    }
}
