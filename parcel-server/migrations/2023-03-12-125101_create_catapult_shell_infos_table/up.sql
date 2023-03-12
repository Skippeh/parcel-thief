CREATE TABLE mission_catapult_shell_infos (
    mission_id VARCHAR PRIMARY KEY REFERENCES missions(id) ON DELETE CASCADE,
    local_id INTEGER NOT NULL,
    x INTEGER NOT NULL,
    y INTEGER NOT NULL,
    z INTEGER NOT NULL
);