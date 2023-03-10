-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS missions_mission_type_idx,
missions_online_mission_type_idx,
missions_progress_state_idx,
missions_qpid_id_idx;