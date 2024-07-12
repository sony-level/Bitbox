-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS email_confirmations;

DROP INDEX IF EXISTS idx_email_confirmations_email;
DROP INDEX IF EXISTS idx_email_confirmations_token;
-- This is the end of the file. Add your down SQL above this line
