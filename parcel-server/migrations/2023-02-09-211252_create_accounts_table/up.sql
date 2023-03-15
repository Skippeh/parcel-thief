CREATE TABLE accounts (
    id VARCHAR PRIMARY KEY,
    display_name VARCHAR NOT NULL,
    provider INTEGER NOT NULL,
    provider_id VARCHAR NOT NULL,
    last_login_date TIMESTAMP NOT NULL,
    UNIQUE(provider, provider_id)
);

CREATE INDEX idx_provider_and_id ON accounts (provider, provider_id);