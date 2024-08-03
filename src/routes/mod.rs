use axum::Router;
use axum::middleware::from_fn;
use crate::constants::route_paths;
use crate::middleware::api_auth::api_key_auth;
use crate::startup::AppState;

mod health_check;
mod homepage;
mod auth;
mod protected;
mod recipes;
mod api;

pub fn homepage_routes() -> Router {
    Router::new().nest(route_paths::ROOT, homepage::routes())
}

pub fn auth_routes() -> Router {
    Router::new().nest(route_paths::ROOT, auth::routes())
}

pub fn health_check_routes() -> Router {
    Router::new().nest(route_paths::HEALTH, health_check::routes())
}

pub fn protected_routes() -> Router {
    Router::new().nest(route_paths::PROTECTED, protected::routes())
}

pub fn recipe_routes() -> Router {
    Router::new().nest(route_paths::RECIPES, recipes::routes::routes())
}

pub fn api_routes(state: &AppState) -> Router {
    Router::new().nest(route_paths::API,api::chopper::routes(state))
}
