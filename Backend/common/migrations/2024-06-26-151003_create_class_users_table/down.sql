-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_class_users_class_id;
DROP INDEX IF EXISTS idx_class_users_user_id;
DROP TABLE IF EXISTS class_users;
