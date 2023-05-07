CREATE TABLE roads (
    id VARCHAR PRIMARY KEY,
    area_hash INTEGER NOT NULL,
    creator_id VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE ON UPDATE CASCADE,
    qpid_start_id INTEGER NOT NULL,
    qpid_end_id INTEGER NOT NULL,
    location_start_id INTEGER NOT NULL,
    location_end_id INTEGER NOT NULL,
    max_height_difference INTEGER NOT NULL,
    length INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL,
    data_version INTEGER NOT NULL
);

CREATE TABLE road_via_qpids (
    road_id VARCHAR NOT NULL REFERENCES roads(id) ON DELETE CASCADE ON UPDATE CASCADE,
    qpid_id INTEGER NOT NULL,
    sort_order INTEGER NOT NULL,
    PRIMARY KEY (road_id, qpid_id, sort_order)
);

CREATE TABLE road_data (
    road_id VARCHAR PRIMARY KEY NOT NULL REFERENCES roads(id) ON DELETE CASCADE ON UPDATE CASCADE,
    data BYTEA NOT NULL
);