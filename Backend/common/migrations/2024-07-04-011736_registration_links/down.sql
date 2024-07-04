-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS registration_links;

DROP INDEX IF EXISTS idx_registration_links_user_id;
DROP INDEX IF EXISTS idx_registration_links_project_id;
DROP INDEX IF EXISTS idx_registration_links_group_id;
DROP INDEX IF EXISTS idx_registration_links_class_id;
DROP INDEX IF EXISTS idx_registration_links_used_by;



