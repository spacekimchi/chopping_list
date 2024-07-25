use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
}

#[derive(Debug, FromRow)]
pub struct CreateUserParams {
    pub email: String,
    pub password_hash: String,
}

impl CreateUserParams {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            email,
            password_hash,
        }
    }

    pub fn new_with_default_password(email: String) -> Self {
        Self {
            email,
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
        let user = sqlx::query_as("INSERT INTO users (email, password_hash) VALUES ($1, $2) RETURNING *")
            .bind(create_user_params.email.clone())
            .bind(create_user_params.password_hash.clone())
            .fetch_optional(db)
            .await?;
        Ok(user)
    }
}
