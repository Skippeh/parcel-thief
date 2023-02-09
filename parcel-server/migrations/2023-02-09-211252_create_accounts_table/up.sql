-- Create accounts table
CREATE TABLE accounts (
    id CHAR(32) PRIMARY KEY,
    steam_id BIGINT NOT NULL,
    UNIQUE(steam_id)
);

CREATE INDEX idx_steam_id ON accounts (steam_id);