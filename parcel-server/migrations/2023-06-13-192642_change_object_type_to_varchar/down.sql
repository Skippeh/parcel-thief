-- This file should undo anything in `up.sql`
-- Add new column for object_type that is nullable initially (we'll remove nullable after values are migrated)
ALTER TABLE
    qpid_objects
ADD
    COLUMN object_type_new INTEGER;

UPDATE
    qpid_objects
SET
    object_type_new = (
        CASE
            WHEN object_type = 'm' THEN 0
            WHEN object_type = 'z' THEN 1
            WHEN object_type = 'c' THEN 2
            WHEN object_type = 'p' THEN 3
            WHEN object_type = 'a' THEN 4
            WHEN object_type = 'r' THEN 5
            WHEN object_type = 'l' THEN 6
            WHEN object_type = 's' THEN 7
            WHEN object_type = 'w' THEN 8
            WHEN object_type = 'b' THEN 9
            WHEN object_type = 't' THEN 10
            WHEN object_type = 'v' THEN 11
            WHEN object_type = 'k' THEN 12
            WHEN object_type = 'n' THEN 13
            WHEN object_type = 'h' THEN 14
            WHEN object_type = 'e' THEN 15
            WHEN object_type = 'u' THEN 16
            WHEN object_type = 'i' THEN 17
            WHEN object_type = 'o' THEN 18
            WHEN object_type = 'x' THEN 19
            WHEN object_type = 'B' THEN 20
            WHEN object_type = 'R' THEN 21
            WHEN object_type = 'S' THEN 22
        END
    );

-- Remove nullability from object_type_new
ALTER TABLE
    qpid_objects
ALTER COLUMN
    object_type_new
SET
    NOT NULL;

-- Delete object_type column
ALTER TABLE
    qpid_objects DROP COLUMN object_type;

-- Rename object_type_new to object_type
ALTER TABLE
    qpid_objects RENAME COLUMN object_type_new TO object_type;