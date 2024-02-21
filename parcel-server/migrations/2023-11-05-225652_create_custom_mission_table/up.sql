-- Create custom_missions table
CREATE TABLE custom_missions (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY (START WITH 1 INCREMENT BY 1),
    creator_id BIGINT NOT NULL REFERENCES frontend_accounts(id) ON DELETE CASCADE ON UPDATE CASCADE,
    type SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create custom_mission_rewards table
-- Note that we don't need a table for delivery/recovery cargo since it can be calculated from the 'normal' missions
CREATE TABLE custom_mission_rewards (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    custom_mission_id BIGINT REFERENCES custom_missions(id) ON DELETE CASCADE ON UPDATE CASCADE,
    item_hash INTEGER NOT NULL,
    amount INTEGER NOT NULL,
    UNIQUE (custom_mission_id, item_hash)
);

-- Create custom_mission_collection_cargo table
-- This table is used to store cargo type/amount and state for collection missions
CREATE TABLE custom_mission_collection_cargo (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    custom_mission_id BIGINT REFERENCES custom_missions(id) ON DELETE CASCADE ON UPDATE CASCADE,
    item_hash INTEGER NOT NULL,
    target_amount INTEGER NOT NULL,
    current_amount INTEGER NOT NULL,
    UNIQUE (custom_mission_id, item_hash)
);

-- Add custom_mission_id column to missions
ALTER TABLE
    missions
ADD
    COLUMN custom_mission_id BIGINT REFERENCES custom_missions(id) ON DELETE CASCADE ON UPDATE CASCADE;