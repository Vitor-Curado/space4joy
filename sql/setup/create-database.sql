
\getenv app_db_password APP_DB_PASSWORD

-- Refuse to overwrite existing resources.
SELECT EXISTS (
    SELECT 1 FROM pg_roles WHERE rolname = :'app_db_user'
) AS role_exists \gset

SELECT EXISTS (
    SELECT 1 FROM pg_database WHERE datname = :'app_db_name'
) AS database_exists \gset

\if :role_exists
    \echo 'ERROR: The requested application role already exists.'
    \quit 1
\endif

\if :database_exists
    \echo 'ERROR: The requested database already exists.'
    \quit 1
\endif

-- Create the application role.
SELECT format(
    'CREATE ROLE %I LOGIN PASSWORD %L',
    :'app_db_user',
    :'app_db_password'
) \gexec

-- Create the database and assign ownership.
SELECT format(
    'CREATE DATABASE %I OWNER %I',
    :'app_db_name',
    :'app_db_user'
) \gexec
