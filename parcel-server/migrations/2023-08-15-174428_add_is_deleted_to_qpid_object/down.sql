-- This file should undo anything in `up.sql`
ALTER TABLE
    qpid_objects DROP COLUMN is_deleted;