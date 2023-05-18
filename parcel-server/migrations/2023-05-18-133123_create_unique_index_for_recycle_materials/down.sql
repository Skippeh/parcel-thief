-- This file should undo anything in `up.sql`
-- Re-create the old index
CREATE INDEX qpid_object_recycle_materials_object_id_idx ON qpid_object_recycle_materials (object_id);

-- Make contributor_id non-nullable. Note that this will fail if there's any existing data with null contributor_id.
ALTER TABLE
    qpid_object_recycle_materials
ALTER COLUMN
    contributor_id
SET
    NOT NULL;

-- Drop null index
DROP INDEX qpid_object_recycle_materials_object_id_null_contributor_id_idx;

-- Drop the new index
DROP INDEX qpid_object_recycle_materials_object_id_contributor_id_idx;

-- Rename contributor_id to contributor
ALTER TABLE
    qpid_object_recycle_materials RENAME COLUMN contributor_id TO contributor;