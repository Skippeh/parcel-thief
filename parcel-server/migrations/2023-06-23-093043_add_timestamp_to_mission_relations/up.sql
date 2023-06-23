-- Your SQL goes here
ALTER TABLE
    mission_relations
ADD
    COLUMN updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP;

CREATE INDEX mission_relations_updated_at_idx ON mission_relations (updated_at ASC);