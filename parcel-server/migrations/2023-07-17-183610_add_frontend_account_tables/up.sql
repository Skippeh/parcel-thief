-- Basic frontend account properties, applies for all accounts regardless of if they are local or not, or have a provider connection or not
CREATE TABLE frontend_accounts (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    -- Game account id needs to be optional otherwise admin-only accounts would not be possible
    game_account_id VARCHAR UNIQUE REFERENCES accounts(id) ON DELETE CASCADE ON UPDATE CASCADE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    permissions BIGINT NOT NULL
);

-- Credentials for logging in locally to a frontend account
CREATE TABLE frontend_account_credentials (
    account_id BIGINT PRIMARY KEY REFERENCES frontend_accounts(id) ON DELETE CASCADE ON UPDATE CASCADE,
    username VARCHAR UNIQUE NOT NULL,
    password VARCHAR NOT NULL,
    salt BYTEA NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Provider connections for frontend accounts
-- Allows a provider account to be connected to a frontend account
CREATE TABLE frontend_account_provider_connections (
    account_id BIGINT PRIMARY KEY REFERENCES frontend_accounts(id) ON DELETE CASCADE ON UPDATE CASCADE,
    provider INTEGER NOT NULL,
    provider_id VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (provider, provider_id)
);