-- Your SQL goes here
CREATE UNIQUE INDEX mission_relations_mission_id_account_id_idx ON mission_relations (mission_id, account_id);