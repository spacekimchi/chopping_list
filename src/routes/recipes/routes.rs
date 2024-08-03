use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post}, Router,
    extract::{Path, Json},
};
use axum::Extension;
use axum::response::Html;
use serde::Deserialize;
use crate::startup::AppState;
use crate::template_helpers::{render_content, RenderTemplateParams, err_500_template};

use crate::user::AuthSession;
use crate::models::recipe::Recipe;
use crate::utils::e500;
use crate::constants::{
    route_paths,
    html_templates,
};

pub fn routes() -> Router {
    Router::new()
        .route(route_paths::ROOT, get(self::get::index))
        .route("/:recipe_id", get(self::get::show))
}

#[derive(Debug, Deserialize)]
pub struct ExtensionRecipeParams {
    pub content: String,
}

mod post {
    use super::*;

    pub async fn create_from_extension(
        mut auth_session: AuthSession,
        Extension(state): Extension<AppState>,
        Json(recipe_params): Json<ExtensionRecipeParams>,
    ) -> impl IntoResponse {
        let user = match auth_session.user {
            Some(user) => user,
            None => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
        };



        "".into_response()
    }
}

mod get {
    use super::*;

    pub async fn index(auth_session: AuthSession, Extension(state): Extension<AppState>) -> impl IntoResponse {
        let user = match auth_session.user {
            Some(user) => user,
            None => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
        };
        let recipes = match user.get_recipes(&state.db).await.map_err(e500) {
            Ok(recipes) => recipes,
            Err(err) => return err.into_response()
        };

        let mut context = tera::Context::new();
        let boo = "FROM PROTECTED ROUTE";
        context.insert("recipes", &recipes);
        context.insert("boo", &boo);
        match render_content(
            &RenderTemplateParams::new(html_templates::RECIPES_INDEX, &state.tera)
            .with_context(&context)
        ) {
            Ok(homepage_template) => Html(homepage_template).into_response(),
            Err(e) => e.into_response()
        }
    }

    pub async fn show(
        auth_session: AuthSession,
        Extension(state): Extension<AppState>,
        Path(recipe_id): Path<i32>,
    ) -> impl IntoResponse {
        let user = match auth_session.user {
            Some(user) => user,
            None => return StatusCode::INTERNAL_SERVER_ERROR.into_response()
        };
        let recipe = match Recipe::get_full_recipe_details(&state.db, &user.id, recipe_id).await {
            Ok(recipe_full_details) => recipe_full_details,
            Err(err) => return Html(err_500_template(&state.tera, err)).into_response()
        };
        let mut context = tera::Context::new();
        context.insert("recipe", &recipe);
        match render_content(
            &RenderTemplateParams::new(html_templates::RECIPES_SHOW, &state.tera)
            .with_context(&context)
        ).map_err(e500) {
            Ok(homepage_template) => Html(homepage_template).into_response(),
            Err(err) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, Html(err_500_template(&state.tera, err))).into_response()
            }
        }
    }
}
