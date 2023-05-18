-- Rename contributor column to contributor_id
ALTER TABLE
    qpid_object_recycle_materials RENAME COLUMN contributor TO contributor_id;

-- Make contributor_id nullable
ALTER TABLE
    qpid_object_recycle_materials
ALTER COLUMN
    contributor_id DROP NOT NULL;

-- Create unique index for (object id, contributor_id)
CREATE UNIQUE INDEX qpid_object_recycle_materials_object_id_contributor_id_idx ON qpid_object_recycle_materials (object_id, contributor_id);

-- Create unique index that only allows one null contributor_id
CREATE UNIQUE INDEX qpid_object_recycle_materials_object_id_null_contributor_id_idx ON qpid_object_recycle_materials (object_id, (contributor_id IS NULL))
WHERE
    contributor_id IS NULL;

-- Drop the old index
DROP INDEX qpid_object_recycle_materials_object_id_idx;