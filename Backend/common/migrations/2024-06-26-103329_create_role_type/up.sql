-- Your SQL goes here
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'user_role') THEN
        CREATE TYPE user_role AS ENUM ('trainer', 'student');
    END IF;
END
$$; 
