-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_notifications_user_id;
DROP TABLE IF EXISTS notifications;
