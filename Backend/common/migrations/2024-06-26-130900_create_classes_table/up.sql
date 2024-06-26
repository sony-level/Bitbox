-- Your SQL goes here
CREATE TABLE IF NOT EXISTS classes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_classes_name ON classes(name);
COMMENT ON TABLE classes IS 'Stocke les données des classes';

-- Compare this snippet from Bitbox/Backend/common/migrations/2024-06-26-130801_create_users_table/down.sql: