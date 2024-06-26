-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_evaluation_results_user_id;
DROP INDEX IF EXISTS idx_evaluation_results_group_id;
DROP INDEX IF EXISTS idx_evaluation_results_project_id;
DROP TABLE IF EXISTS evaluation_results;
