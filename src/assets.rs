use crate::{
    constants::{descriptions, icons, paths::VITE_MANIFEST, routes, titles},
    error::{AppError, AppResult},
    templates::{
        AboutTemplate, BaseTemplateContext, ConnectTemplate, ExperiencesTemplate, HomeTemplate,
        ServicesTemplate, ThankYouTemplate,
    },
};
use askama::Template;
use serde::Deserialize;
use std::{fs, sync::Arc};

#[derive(Debug, Deserialize)]
struct ViteManifestEntry {
    // The file is actually the JS, don't blame me blame Vite
    file: String,
    #[serde(default)]
    css: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ViteManifest {
    #[serde(rename = "index.html")]
    index: ViteManifestEntry,
}

#[derive(Clone)]
pub struct Assets {
    pub css: String,
    pub js: String,
}

impl Assets {
    pub fn new() -> AppResult<Self> {
        let content = fs::read_to_string(VITE_MANIFEST)?;
        let manifest: ViteManifest = serde_json::from_str(&content)?;

        Ok(Self {
            css: manifest
                .index
                .css
                .first()
                .cloned()
                .ok_or(AppError::MissingAsset("index.css"))?,
            js: manifest.index.file,
        })
    }

    pub fn generate_all_static_pages(&self) -> AppResult<()> {
        let assets = Arc::new(self.clone());
        self.generate_about(assets.clone())?;
        self.generate_experiences(assets.clone())?;
        self.generate_home(assets.clone())?;
        self.generate_services(assets.clone())?;
        self.generate_connect(assets.clone())?;
        self.generate_thanks(assets)
    }

    fn generate_about(&self, assets: Arc<Assets>) -> AppResult<()> {
        let base = BaseTemplateContext::new(
            titles::ABOUT,
            descriptions::ABOUT,
            routes::ABOUT.into(),
            icons::CONTACT,
            assets,
        );
        let template = AboutTemplate { base };
        self.write_static_page("about", template)
    }

    fn generate_experiences(&self, assets: Arc<Assets>) -> AppResult<()> {
        let base = BaseTemplateContext::new(
            titles::EXPERIENCES,
            descriptions::EXPERIENCES,
            routes::EXPERIENCES.into(),
            icons::HOME,
            assets,
        );
        let template = ExperiencesTemplate { base };
        self.write_static_page("experiences", template)
    }

    fn generate_home(&self, assets: Arc<Assets>) -> AppResult<()> {
        let base = BaseTemplateContext::new(
            titles::HOME,
            descriptions::HOME,
            routes::HOME.into(),
            icons::HOME,
            assets,
        );
        let template = HomeTemplate { base };
        self.write_static_page("home", template)
    }

    fn generate_services(&self, assets: Arc<Assets>) -> AppResult<()> {
        let base = BaseTemplateContext::new(
            titles::SERVICES,
            descriptions::SERVICES,
            routes::SERVICES.into(),
            icons::HOME,
            assets,
        );
        let template = ServicesTemplate { base };
        self.write_static_page("services", template)
    }

    fn generate_connect(&self, assets: Arc<Assets>) -> AppResult<()> {
        let base = BaseTemplateContext::new(
            titles::CONNECT,
            descriptions::CONNECT,
            routes::CONNECT.into(),
            icons::HOME,
            assets,
        );
        let template = ConnectTemplate { base };
        self.write_static_page("connect", template)
    }

    fn generate_thanks(&self, assets: Arc<Assets>) -> AppResult<()> {
        let base = BaseTemplateContext::new(
            titles::THANK_YOU,
            descriptions::THANK_YOU,
            routes::THANK_YOU.into(),
            icons::HOME,
            assets,
        );
        let template = ThankYouTemplate { base };
        self.write_static_page("thanks", template)
    }

    fn write_static_page<T: Template>(&self, route: &str, template: T) -> AppResult<()> {
        let directory = format!("static/{route}");
        fs::create_dir_all(&directory)?;
        let html = template.render()?;
        fs::write(format!("{directory}/index.html"), html)?;

        Ok(())
    }
}
