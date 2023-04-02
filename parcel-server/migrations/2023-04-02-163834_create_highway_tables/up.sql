CREATE TABLE devoted_highway_resources (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    account_id VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    construction_id INTEGER NOT NULL,
    time TIMESTAMP NOT NULL,
    resource_id SMALLINT NOT NULL,
    num_resources INTEGER NOT NULL
);

CREATE INDEX idx_devoted_highway_resources_time ON devoted_highway_resources (time);

CREATE TABLE total_highway_resources (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    construction_id INTEGER,
    resource_id SMALLINT NOT NULL,
    num_resources BIGINT NOT NULL DEFAULT 0
);

CREATE UNIQUE INDEX idx_total_highway_resources_construction_id_resource_id ON total_highway_resources (construction_id, resource_id);