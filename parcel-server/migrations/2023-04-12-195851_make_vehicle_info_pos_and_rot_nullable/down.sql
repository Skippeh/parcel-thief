-- This file should undo anything in `up.sql`
ALTER TABLE
    qpid_object_vehicle_infos
ALTER COLUMN
    new_pos_x
SET
    NOT NULL;

ALTER TABLE
    qpid_object_vehicle_infos
ALTER COLUMN
    new_pos_y
SET
    NOT NULL;

ALTER TABLE
    qpid_object_vehicle_infos
ALTER COLUMN
    new_pos_z
SET
    NOT NULL;

ALTER TABLE
    qpid_object_vehicle_infos
ALTER COLUMN
    new_rot_x
SET
    NOT NULL;

ALTER TABLE
    qpid_object_vehicle_infos
ALTER COLUMN
    new_rot_y
SET
    NOT NULL;

ALTER TABLE
    qpid_object_vehicle_infos
ALTER COLUMN
    new_rot_z
SET
    NOT NULL;