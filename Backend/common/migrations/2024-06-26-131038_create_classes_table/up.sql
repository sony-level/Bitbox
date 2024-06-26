-- Your SQL goes here
CREATE TABLE IF NOT EXISTS classes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_classes_name ON classes(name);

-- Compare this snippet from Bitbox/Backend/common/migrations/2024-06-26-130801_create_users_table/down.sql: