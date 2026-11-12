use crate::{assets::Assets, state::AppState};
use askama::Template;
use std::sync::Arc;

#[derive(Clone)]
pub struct SeoMetadata {
    pub title: String,
    pub description: String,
    pub canonical_url: String,
}

#[derive(Clone)]
pub struct BaseTemplateContext {
    pub seo: SeoMetadata,
    pub favicon: &'static str,
    pub assets: Arc<Assets>,
}

impl BaseTemplateContext {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        canonical_url: String,
        favicon: &'static str,
        assets: Arc<Assets>,
    ) -> Self {
        Self {
            seo: SeoMetadata {
                title: title.into(),
                description: description.into(),
                canonical_url,
            },
            favicon,
            assets,
        }
    }

    pub fn build_base_context(
        state: &AppState,
        title: impl Into<String>,
        description: impl Into<String>,
        canonical_path: &str,
        favicon: &'static str,
    ) -> BaseTemplateContext {
        let origin = state.config.app.public_base_url.trim_end_matches('/');
        let canonical_url = format!("{origin}{canonical_path}");

        BaseTemplateContext::new(
            title,
            description,
            canonical_url,
            favicon,
            state.assets.clone(),
        )
    }
}

#[derive(Template)]
#[template(path = "pages/index.html")]
pub struct IndexTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/experiences.html")]
pub struct Space4joyExperienceTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/services.html")]
pub struct AtYourServiceTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/contact.html")]
pub struct ContactTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/about.html")]
pub struct AboutTemplate {
    pub base: BaseTemplateContext,
}
