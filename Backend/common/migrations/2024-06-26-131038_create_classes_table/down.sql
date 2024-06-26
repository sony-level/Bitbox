-- This file should undo anything in `up.sql`
 DROP INDEX IF EXISTS idx_classes_name;
DROP TABLE IF EXISTS classes;

-- Compare this snippet from Bitbox/Backend/common/migrations/2024-06-26-130801_create_users_table/down.sql: