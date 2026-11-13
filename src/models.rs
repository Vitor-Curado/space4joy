use serde::Deserialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ContactSubmission {
    pub name: String,
    pub email: String,
    pub country: String,
    pub city: String,
    pub message: String,

    #[serde(default)]
    pub marketing_consent: bool,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub country: String,
    pub city: String,
    pub marketing_consent: bool,
    pub consent_recorded_at: Option<OffsetDateTime>,
    pub consent_source: Option<String>,
    pub unsubscribed_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Message {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub message: String,
    pub created_at: OffsetDateTime,
}
