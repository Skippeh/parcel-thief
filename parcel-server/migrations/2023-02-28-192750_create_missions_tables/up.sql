CREATE TABLE missions (
    id VARCHAR PRIMARY KEY,
    area_id INTEGER NOT NULL,
    creator_id VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    worker_id VARCHAR REFERENCES accounts(id) ON DELETE CASCADE,
    qpid_id INTEGER NOT NULL,
    qpid_start_location INTEGER NOT NULL,
    qpid_end_location INTEGER NOT NULL,
    qpid_delivered_location INTEGER,
    mission_static_id BIGINT NOT NULL,
    mission_type INTEGER NOT NULL,
    online_mission_type INTEGER NOT NULL,
    progress_state INTEGER NOT NULL,
    registered_time TIMESTAMP NOT NULL,
    expiration_time TIMESTAMP NOT NULL
);

CREATE TABLE mission_supply_infos (
    mission_id VARCHAR PRIMARY KEY REFERENCES missions(id) ON DELETE CASCADE,
    item_hash BIGINT NOT NULL,
    amount INTEGER NOT NULL
);

CREATE TABLE mission_dynamic_mission_infos (
    mission_id VARCHAR PRIMARY KEY REFERENCES missions(id) ON DELETE CASCADE,
    client_name_hash INTEGER NOT NULL,
    reward_name_hash INTEGER NOT NULL
);

CREATE TABLE mission_dynamic_location_infos (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    mission_id VARCHAR NOT NULL REFERENCES missions(id) ON DELETE CASCADE,
    -- there's 3 possible types of this for each mission.
    -- 1. start info
    -- 2. end info
    -- 3. delivered info
    type SMALLINT NOT NULL,
    location_id VARCHAR NOT NULL,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    z INTEGER NOT NULL
);

CREATE UNIQUE INDEX mission_dynamic_location_infos_mission_id_type_idx ON mission_dynamic_location_infos (mission_id, type);

CREATE TABLE mission_baggages (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    mission_id VARCHAR NOT NULL REFERENCES missions(id) ON DELETE CASCADE,
    amount INTEGER NOT NULL,
    name_hash INTEGER NOT NULL,
    user_index INTEGER NOT NULL,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    z INTEGER NOT NULL,
    is_returned BOOLEAN NOT NULL DEFAULT false
);

CREATE INDEX mission_baggage_mission_id ON mission_baggages (mission_id);

CREATE TABLE mission_baggage_ammo_infos (
    baggage_id BIGINT PRIMARY KEY REFERENCES mission_baggages(id) ON DELETE CASCADE,
    ammo_id VARCHAR NOT NULL,
    clip_count SMALLINT NOT NULL,
    count SMALLINT NOT NULL
);

CREATE TABLE mission_relations (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    mission_id VARCHAR NOT NULL REFERENCES missions(id) ON DELETE CASCADE,
    account_id VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE INDEX mission_relations_mission_id_idx ON mission_relations (mission_id);