-- Add new column for object_type that is nullable initially (we'll remove nullable after values are migrated)
ALTER TABLE
    qpid_objects
ADD
    COLUMN object_type_new VARCHAR;

UPDATE
    qpid_objects
SET
    object_type_new = (
        CASE
            WHEN object_type = 0 THEN 'm'
            WHEN object_type = 1 THEN 'z'
            WHEN object_type = 2 THEN 'c'
            WHEN object_type = 3 THEN 'p'
            WHEN object_type = 4 THEN 'a'
            WHEN object_type = 5 THEN 'r'
            WHEN object_type = 6 THEN 'l'
            WHEN object_type = 7 THEN 's'
            WHEN object_type = 8 THEN 'w'
            WHEN object_type = 9 THEN 'b'
            WHEN object_type = 10 THEN 't'
            WHEN object_type = 11 THEN 'v'
            WHEN object_type = 12 THEN 'k'
            WHEN object_type = 13 THEN 'n'
            WHEN object_type = 14 THEN 'h'
            WHEN object_type = 15 THEN 'e'
            WHEN object_type = 16 THEN 'u'
            WHEN object_type = 17 THEN 'i'
            WHEN object_type = 18 THEN 'o'
            WHEN object_type = 19 THEN 'x'
            WHEN object_type = 20 THEN 'B'
            WHEN object_type = 21 THEN 'R'
            WHEN object_type = 22 THEN 'S'
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