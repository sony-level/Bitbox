-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_groups_name;
DROP INDEX IF EXISTS idx_groups_project_id;
DROP TABLE IF EXISTS groups;
