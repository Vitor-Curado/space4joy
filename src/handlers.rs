use crate::{
    constants::routes,
    error::{AppError, AppResult},
    models::ContactSubmission,
    repository,
    state::AppState,
};

use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect, Response},
};

use askama::Template;
use std::time::Instant;

#[allow(clippy::needless_pass_by_value)]
pub fn render_template<T: Template>(t: T) -> AppResult<Response> {
    // Timing instrumentation to measure render cost
    let start = Instant::now();
    let html = t.render().map_err(|e| {
        tracing::error!(
            error = ?e,
            template = std::any::type_name::<T>(),
            "Template render failed");
        e
    })?;
    tracing::debug!("Template render took {:?}", start.elapsed());
    Ok(Html(html).into_response())
}

pub async fn create_contact(
    State(state): State<AppState>,
    Form(mut submission): Form<ContactSubmission>,
) -> AppResult<Redirect> {
    // Normalize incoming data.
    submission.name = submission.name.trim().to_string();
    submission.email = submission.email.trim().to_ascii_lowercase();
    submission.country = submission.country.trim().to_string();
    submission.city = submission.city.trim().to_string();
    submission.message = submission.message.trim().to_string();

    // Basic validation.
    if submission.name.is_empty()
        || submission.email.is_empty()
        || submission.country.is_empty()
        || submission.city.is_empty()
        || submission.message.is_empty()
    {
        return Err(AppError::BadRequest(
            "All contact fields are required.".to_string(),
        ));
    }

    if submission.name.len() > 120
        || submission.email.len() > 254
        || submission.country.len() > 100
        || submission.city.len() > 100
        || submission.message.len() > 5000
    {
        return Err(AppError::BadRequest(
            "One or more fields exceed the permitted length.".to_string(),
        ));
    }

    if !submission.email.contains('@') {
        return Err(AppError::BadRequest("Invalid email address.".to_string()));
    }

    // Persist contact and message.
    repository::create_contact_and_message(&state.db, &submission).await?;

    // Post/Redirect/Get.
    Ok(Redirect::to(routes::THANK_YOU))
}
