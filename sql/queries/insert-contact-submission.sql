WITH upserted_contact AS (
    INSERT INTO contacts (
        id,
        name,
        email,
        country,
        city,
        marketing_consent,
        consent_recorded_at,
        consent_source
    )
    VALUES (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        CASE WHEN $6 THEN NOW() ELSE NULL END,
        CASE WHEN $6 THEN 'connect_form' ELSE NULL END
    )
    ON CONFLICT (LOWER(email))
    DO UPDATE SET
        name = EXCLUDED.name,
        country = EXCLUDED.country,
        city = EXCLUDED.city,

        -- Only an explicit opt-in changes consent.
        marketing_consent =
            contacts.marketing_consent OR EXCLUDED.marketing_consent,

        consent_recorded_at =
            CASE
                WHEN EXCLUDED.marketing_consent THEN NOW()
                ELSE contacts.consent_recorded_at
            END,

        consent_source =
            CASE
                WHEN EXCLUDED.marketing_consent THEN 'connect_form'
                ELSE contacts.consent_source
            END,

        unsubscribed_at =
            CASE
                WHEN EXCLUDED.marketing_consent THEN NULL
                ELSE contacts.unsubscribed_at
            END

    RETURNING id
)

INSERT INTO messages (
    id,
    contact_id,
    message
)
SELECT
    $7,
    id,
    $8
FROM upserted_contact;