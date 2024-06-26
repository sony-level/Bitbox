-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_evaluations_evaluator_id;
DROP INDEX IF EXISTS idx_evaluations_evaluatee_id;
DROP INDEX IF EXISTS idx_evaluations_project_id;
DROP INDEX IF EXISTS idx_evaluations_group_id;

DROP TABLE IF EXISTS evaluations;

