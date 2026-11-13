use crate::{constants::routes, handlers::create_contact, state::AppState};
use axum::{
    Router,
    routing::{get},
    response::Redirect
};

pub fn public_routes() -> Router<AppState> {
    Router::new().route(
        routes::CONNECT,
        get(|| async { Redirect::permanent("/connect/") }).post(create_contact)
    )
}
