-- This file should undo anything in `up.sql`
ALTER TABLE
    custom_mission_rewards
ALTER COLUMN
    custom_mission_id DROP NOT NULL