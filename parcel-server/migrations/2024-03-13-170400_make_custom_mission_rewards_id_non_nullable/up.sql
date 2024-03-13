-- Make custom_mission_rewards.custom_mission_id non-nullable
ALTER TABLE
    custom_mission_rewards
ALTER COLUMN
    custom_mission_id
SET
    NOT NULL