use crate::{
    constants::{self, endpoints}, handlers::{home, space4joy_experience, at_your_service}, state::AppState,
};
use axum::{Router, routing::get};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route(constants::endpoints::HOME, get(home))
        .route(
            constants::endpoints::SPACE4JOYEXPERIENCE,
            get(space4joy_experience),
        )
        .route(endpoints::ATYOURSERVICE, get(at_your_service))
}
