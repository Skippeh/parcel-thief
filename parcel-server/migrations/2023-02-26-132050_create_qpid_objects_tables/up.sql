CREATE TABLE qpid_objects (
    id VARCHAR PRIMARY KEY,
    -- account id of creator
    creator_id VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    exponent INTEGER NOT NULL,
    -- this is technically u32 but we're storing as i64 since postgres doesn't have unsigned types
    likes BIGINT NOT NULL,
    pos_x INTEGER NOT NULL,
    pos_y INTEGER NOT NULL,
    pos_z INTEGER NOT NULL,
    rot_x INTEGER NOT NULL,
    rot_y INTEGER NOT NULL,
    rot_z INTEGER NOT NULL,
    grid_x INTEGER NOT NULL,
    grid_y INTEGER NOT NULL,
    area_id INTEGER NOT NULL,
    qpid_id INTEGER NOT NULL,
    object_type INTEGER NOT NULL,
    sub_type VARCHAR NOT NULL,
    updated_time TIMESTAMP NOT NULL
);

CREATE TABLE qpid_object_construction_materials (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    object_id VARCHAR NOT NULL REFERENCES qpid_objects(id) ON DELETE CASCADE,
    contributor VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    mats_0 INTEGER NOT NULL DEFAULT 0,
    mats_1 INTEGER NOT NULL DEFAULT 0,
    mats_2 INTEGER NOT NULL DEFAULT 0,
    mats_3 INTEGER NOT NULL DEFAULT 0,
    mats_4 INTEGER NOT NULL DEFAULT 0,
    mats_5 INTEGER NOT NULL DEFAULT 0,
    repair_0 INTEGER NOT NULL DEFAULT 0,
    repair_1 INTEGER NOT NULL DEFAULT 0,
    repair_2 INTEGER NOT NULL DEFAULT 0,
    repair_3 INTEGER NOT NULL DEFAULT 0,
    repair_4 INTEGER NOT NULL DEFAULT 0,
    repair_5 INTEGER NOT NULL DEFAULT 0,
    contribute_time TIMESTAMP NOT NULL
);

CREATE INDEX qpid_object_construction_materials_object_id_idx ON qpid_object_construction_materials (object_id);

CREATE TABLE qpid_object_recycle_materials (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    object_id VARCHAR NOT NULL REFERENCES qpid_objects(id) ON DELETE CASCADE,
    contributor VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    mats_0 INTEGER NOT NULL DEFAULT 0,
    mats_1 INTEGER NOT NULL DEFAULT 0,
    mats_2 INTEGER NOT NULL DEFAULT 0,
    mats_3 INTEGER NOT NULL DEFAULT 0,
    mats_4 INTEGER NOT NULL DEFAULT 0,
    mats_5 INTEGER NOT NULL DEFAULT 0,
    recycle_time TIMESTAMP NOT NULL
);

CREATE INDEX qpid_object_recycle_materials_object_id_idx ON qpid_object_recycle_materials (object_id);

CREATE TABLE qpid_object_baggages (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    object_id VARCHAR NOT NULL REFERENCES qpid_objects(id) ON DELETE CASCADE,
    creator VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    item_name_hash INTEGER NOT NULL,
    mission_id INTEGER NOT NULL,
    life INTEGER NOT NULL,
    endurance INTEGER NOT NULL,
    handle INTEGER NOT NULL
);

CREATE INDEX qpid_object_baggages_object_id_idx ON qpid_object_baggages (object_id);

CREATE TABLE qpid_object_comments (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    object_id VARCHAR NOT NULL REFERENCES qpid_objects(id) ON DELETE CASCADE,
    writer VARCHAR NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    likes BIGINT NOT NULL,
    -- value is i8 but is stored as i16 due to postgres not having an i8 type
    parent_index SMALLINT NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    reference_object VARCHAR NOT NULL
);

CREATE INDEX qpid_object_comments_object_id_idx ON qpid_object_comments (object_id);

CREATE TABLE qpid_object_comment_phrases (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    comment_id BIGINT NOT NULL REFERENCES qpid_object_comments(id) ON DELETE CASCADE,
    phrase INTEGER NOT NULL,
    sort_order SMALLINT NOT NULL
);

CREATE INDEX qpid_object_comment_phrases_comment_id_idx ON qpid_object_comment_phrases(comment_id);

CREATE TABLE qpid_object_rope_infos (
    object_id VARCHAR PRIMARY KEY REFERENCES qpid_objects(id) ON DELETE CASCADE,
    pitch INTEGER NOT NULL,
    heading INTEGER NOT NULL,
    len INTEGER NOT NULL
);

CREATE TABLE qpid_object_stone_infos (
    object_id VARCHAR PRIMARY KEY REFERENCES qpid_objects(id) ON DELETE CASCADE,
    resting_count INTEGER NOT NULL
);

CREATE TABLE qpid_object_bridge_infos (
    object_id VARCHAR PRIMARY KEY REFERENCES qpid_objects(id) ON DELETE CASCADE,
    angle INTEGER NOT NULL
);

CREATE TABLE qpid_object_parking_infos (
    object_id VARCHAR PRIMARY KEY REFERENCES qpid_objects(id) ON DELETE CASCADE,
    location_id INTEGER NOT NULL,
    dynamic_location_id VARCHAR NOT NULL,
    current_qpid_id INTEGER NOT NULL,
    is_parking BOOLEAN NOT NULL
);

CREATE TABLE qpid_object_vehicle_infos (
    object_id VARCHAR PRIMARY KEY REFERENCES qpid_objects(id) ON DELETE CASCADE,
    location_id INTEGER NOT NULL,
    dynamic_location_id VARCHAR NOT NULL,
    current_qpid_id INTEGER NOT NULL,
    is_parking BOOLEAN NOT NULL,
    is_lost BOOLEAN NOT NULL,
    is_race BOOLEAN NOT NULL,
    customize_type INTEGER NOT NULL,
    customize_color INTEGER NOT NULL,
    new_pos_x INTEGER NOT NULL,
    new_pos_y INTEGER NOT NULL,
    new_pos_z INTEGER NOT NULL,
    new_rot_x INTEGER NOT NULL,
    new_rot_y INTEGER NOT NULL,
    new_rot_z INTEGER NOT NULL,
    exponent INTEGER NOT NULL
);

CREATE TABLE qpid_object_extra_infos (
    object_id VARCHAR PRIMARY KEY REFERENCES qpid_objects(id) ON DELETE CASCADE,
    alternative_qpid_id INTEGER NOT NULL
);

CREATE TABLE qpid_object_customize_infos (
    object_id VARCHAR PRIMARY KEY REFERENCES qpid_objects(id) ON DELETE CASCADE,
    -- the following two columns are u32 but are stored as i32 since postgres does not have unsigned types
    customize_param INTEGER NOT NULL,
    customize_color INTEGER NOT NULL
);

CREATE TABLE qpid_object_tags (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    object_id VARCHAR NOT NULL REFERENCES qpid_objects(id) ON DELETE CASCADE,
    tag VARCHAR NOT NULL
);

CREATE INDEX qpid_object_tags_object_id_idx ON qpid_object_tags (object_id);

CREATE UNIQUE INDEX qpid_object_tags_object_id_tag_idx ON qpid_object_tags (object_id, tag)