CREATE TABLE contacts (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    country TEXT NOT NULL,
    city TEXT NOT NULL,
    marketing_consent BOOLEAN NOT NULL DEFAULT FALSE,
    consent_recorded_at TIMESTAMPTZ,
    consent_source TEXT,
    unsubscribed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT contacts_consent_timestamp_check
        CHECK (
            (marketing_consent = TRUE
                AND consent_recorded_at IS NOT NULL
                AND unsubscribed_at IS NULL)
            OR
            (marketing_consent = FALSE)
        )
);

CREATE UNIQUE INDEX contacts_email_unique
ON contacts (LOWER(email));

CREATE TABLE messages (
    id UUID PRIMARY KEY,
    contact_id UUID NOT NULL REFERENCES contacts(id),
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);