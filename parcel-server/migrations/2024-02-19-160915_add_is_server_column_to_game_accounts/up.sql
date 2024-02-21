ALTER TABLE
    accounts
ADD
    COLUMN is_server BOOLEAN NOT NULL DEFAULT FALSE;

CREATE INDEX accounts_is_server_idx ON accounts (is_server);