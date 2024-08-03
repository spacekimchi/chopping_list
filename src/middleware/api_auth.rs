use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
    extract::State,
};
use crate::startup::AppState;
use crate::models::user::User;

pub async fn api_key_auth(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());
    println!("\n\n\nAM I GOING THROUGH API MIDDLEWARE\n\n\n");

    match auth_header {
        Some(auth) if auth.starts_with("Bearer ") => {
            let api_key = &auth[7..];
            if let Some(user) = User::find_by_api_key(&state.db, api_key).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
                req.extensions_mut().insert(user);
                Ok(next.run(req).await)
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        _ => {
            println!("\n\n\nNO AUTHORIZATION TOKEN FOUND\n\n\n");
            Err(StatusCode::UNAUTHORIZED)
        },
    }
}
