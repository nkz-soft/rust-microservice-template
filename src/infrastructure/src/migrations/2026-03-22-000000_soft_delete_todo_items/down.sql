ALTER TABLE to_do_items
DROP COLUMN IF EXISTS deleted_by,
DROP COLUMN IF EXISTS deleted_at;

