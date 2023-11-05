-- This file should undo anything in `up.sql`
-- Delete missions where custom_mission_id is set
DELETE FROM
    missions
WHERE
    custom_mission_id IS NOT NULL;

-- Delete custom_mission_id column
ALTER TABLE
    missions DROP COLUMN IF EXISTS custom_mission_id;

-- Delete custom_mission tables
DROP TABLE IF EXISTS custom_mission_rewards;

DROP TABLE IF EXISTS custom_mission_collection_cargo;

DROP TABLE IF EXISTS custom_missions;