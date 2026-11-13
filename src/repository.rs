use crate::models::ContactSubmission;

use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_contact_and_message(
    pool: &PgPool,
    submission: &ContactSubmission,
) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;

    let contact_id = Uuid::now_v7();
    let message_id = Uuid::now_v7();

    sqlx::query_file!(
        "sql/queries/insert-contact-submission.sql",
        contact_id,
        submission.name,
        submission.email,
        submission.country,
        submission.city,
        submission.marketing_consent,
        message_id,
        submission.message,
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(())
}
