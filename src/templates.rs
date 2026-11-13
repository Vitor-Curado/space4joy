use crate::{assets::Assets, constants::locales, state::AppState};
use askama::Template;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct PageMetadata {
    pub title: String,
    pub description: String,
    pub canonical_url: String,
    pub locale: String,
    pub favicon: &'static str,
    pub json_ld: Vec<Value>,
}

#[derive(Clone)]
pub struct OpenGraphMetadata {
    pub title: String,
    pub description: String,
    pub image: Option<String>,
    pub locale: String,
    pub og_type: &'static str,
}

#[derive(Clone)]
pub struct BaseTemplateContext {
    pub page: PageMetadata,
    pub open_graph: OpenGraphMetadata,
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
        let title = title.into();
        let description = description.into();

        Self {
            page: PageMetadata {
                title: title.clone(),
                description: description.clone(),
                canonical_url,
                locale: locales::ENGLISH.into(),
                favicon,
                json_ld: Vec::new(),
            },
            open_graph: OpenGraphMetadata {
                title,
                description,
                image: None,
                locale: locales::ENGLISH.into(),
                og_type: "website",
            },
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
#[template(path = "pages/home.html")]
pub struct HomeTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/experiences.html")]
pub struct ExperiencesTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/services.html")]
pub struct ServicesTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/about.html")]
pub struct AboutTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/connect.html")]
pub struct ConnectTemplate {
    pub base: BaseTemplateContext,
}

#[derive(Template)]
#[template(path = "pages/thank-you.html")]
pub struct ThankYouTemplate {
    pub base: BaseTemplateContext,
}
