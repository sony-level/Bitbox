-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_group_users_group_id;
DROP TABLE IF EXISTS group_users;
