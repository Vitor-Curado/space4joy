use crate::{
    constants::{self, endpoints, icons, seo_descriptions, titles},
    error::{AppError, AppResult},
    state::AppState,
    templates::{
        AtYourServiceTemplate, BaseTemplateContext, IndexTemplate, Space4joyExperienceTemplate,
    },
};

use axum::{
    extract::State,
    response::{Html, IntoResponse, Response},
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

pub async fn home(State(state): State<AppState>) -> Result<Response, AppError> {
    render_template(IndexTemplate {
        base: BaseTemplateContext::build_base_context(
            &state,
            titles::HOME,
            constants::seo_descriptions::HOME,
            "/",
            icons::HOME,
        ),
    })
}

pub async fn space4joy_experience(State(state): State<AppState>) -> Result<Response, AppError> {
    render_template(Space4joyExperienceTemplate {
        base: BaseTemplateContext::build_base_context(
            &state,
            titles::SPACE4JOYEXPERIENCE,
            seo_descriptions::SPACE4JOYEXPERIENCE,
            endpoints::SPACE4JOYEXPERIENCE,
            icons::HOME
        ),
    })
}

pub async fn at_your_service(State(state): State<AppState>) -> Result<Response, AppError> {
    render_template(AtYourServiceTemplate {
        base: BaseTemplateContext::build_base_context(
            &state,
            titles::ATYOURSERVICE,
            seo_descriptions::ATYOURSERVICE,
            endpoints::ATYOURSERVICE,
            icons::HOME
        ),
    })
}
