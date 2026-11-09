
#!/usr/bin/env bash

set -Eeuo pipefail

PROJECT_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

SQL_SETUP="$PROJECT_ROOT/sql/setup/create-database.sql"
INITIAL_SCHEMA="$PROJECT_ROOT/sql/migrations/initial-schema.sql"

# Check dependencies.
if ! command -v psql >/dev/null 2>&1; then
    printf 'Error: psql is required but was not found.\n' >&2
    exit 1
fi

# Check required files.
for file in "$SQL_SETUP" "$INITIAL_SCHEMA"; do
    if [[ ! -f "$file" ]]; then
        printf 'Error: required file not found: %s\n' "$file" >&2
        exit 1
    fi
done

# Administrative connection.
if [[ -z "${PG_ADMIN_URL:-}" ]]; then
    printf 'Error: PG_ADMIN_URL must be set.\n' >&2
    exit 1
fi

# Collect application database details.
read -r -p "Database name: " APP_DB_NAME
read -r -p "Application role: " APP_DB_USER

if [[ -z "$APP_DB_NAME" || -z "$APP_DB_USER" ]]; then
    printf 'Error: database name and role cannot be empty.\n' >&2
    exit 1
fi

# Read and confirm the application password.
while true; do
    read -r -s -p "Application password: " APP_DB_PASSWORD
    printf '\n'

    read -r -s -p "Confirm password: " APP_DB_PASSWORD_CONFIRM
    printf '\n'

    if [[ -z "$APP_DB_PASSWORD" ]]; then
        printf 'Password cannot be empty. Try again.\n'
    elif [[ "$APP_DB_PASSWORD" != "$APP_DB_PASSWORD_CONFIRM" ]]; then
        printf 'Passwords do not match. Try again.\n'
    else
        break
    fi
done

unset APP_DB_PASSWORD_CONFIRM

# Create the application role and database.
printf '\n==> Creating database and application role\n'

APP_DB_PASSWORD="$APP_DB_PASSWORD" \
psql "$PG_ADMIN_URL" \
    -X \
    -v ON_ERROR_STOP=1 \
    -v app_db_name="$APP_DB_NAME" \
    -v app_db_user="$APP_DB_USER" \
    -f "$SQL_SETUP"

# Apply the initial schema using the newly created application role.
printf '\n==> Applying initial database schema\n'

PGPASSWORD="$APP_DB_PASSWORD" \
psql "$PG_ADMIN_URL" \
    -X \
    -U "$APP_DB_USER" \
    -d "$APP_DB_NAME" \
    -v ON_ERROR_STOP=1 \
    -f "$INITIAL_SCHEMA"

unset APP_DB_PASSWORD

printf '\n==> Database setup completed successfully.\n'
