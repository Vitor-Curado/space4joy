use crate::{
    constants::{self, endpoints}, handlers::{about, at_your_service, home, space4joy_experience}, state::AppState,
};
use axum::{Router, routing::get};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route(constants::endpoints::HOME, get(home))
        .route(
            constants::endpoints::EXPERIENCES,
            get(space4joy_experience),
        )
        .route(endpoints::SERVICES, get(at_your_service))
        .route(endpoints::ABOUT, get(about))
}
