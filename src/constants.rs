// Usually changes

pub mod endpoints {
    pub const HOME: &str = "/";
    pub const CONTACT: &str = "/contact";
    pub const SPACE4JOYEXPERIENCE: &str = "/space4joy-experience";
    pub const ATYOURSERVICE: &str = "/at-your-service";
    pub const ABOUT: &str = "/about";
}

pub mod seo_descriptions {
    pub const HOME: &str = "Less Stress. More Joy. Create Room to THRIVE.";
    pub const SPACE4JOYEXPERIENCE: &str = "Learn the practical ways to experience joy";
    pub const ATYOURSERVICE: &str = "Here, you may learn...";
    pub const CONTACT: &str = "Contact information to get in touch with me";
    pub const ABOUT: &str = "Meet the founder of space4joy";
}

pub mod contact_information {
    pub const EMAIL: &str = "space4joy@gmail.com";
    pub const LINKEDIN: &str = "https://www.linkedin.com/in/petmic/";
    pub const GITHUB: &str = "https://github.com/Vitor-Curado/space4joy";
    pub const WHATSAPP: &str = "https://wa.me/16477817484";
}

pub mod icons {
    pub const CONTACT: &str = "contact.svg";
    pub const HOME: &str = "home.svg";
}

pub mod titles {
    pub const HOME: &str = "Space4Joy";
    pub const SPACE4JOYEXPERIENCE: &str = "Space4Joy experience";
    pub const ATYOURSERVICE: &str = "At your service";
    pub const CONTACT: &str = "Contact";
    pub const ABOUT: &str = "About";
}

// Rarely changes

pub mod paths {
    pub const DIST: &str = "dist";
    pub const CSS: &str = "static/css";
    pub const VITE_MANIFEST: &str = "dist/.vite/manifest.json";
}

pub mod errors {
    pub const SESSION_ERROR: &str = "Session error";
    pub const MISSING_ASSET: &str = "Missing asset";
    pub const CONFIG_ERROR: &str = "Configuration error";
    pub const BAD_REQUEST: &str = "Bad request";
    pub const NOT_FOUND: &str = "Not found";
    pub const UNAUTHORIZED: &str = "Unauthorized";
    pub const INTERNAL: &str = "Internal server error";
}

pub mod cookies {
    pub const SESSION_TOKEN: &str = "session_token";
    pub const SESSION_ID: &str = "session_id";
}

pub mod service {
    pub const NAME: &str = "buildhaven";
}

pub mod project {
    pub const NAME: &str = env!("CARGO_PKG_NAME");
    pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
}
