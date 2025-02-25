-- Your SQL goes here

CREATE TABLE IF NOT EXISTS classes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_by UUID NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_classes_name ON classes(name);
COMMENT ON TABLE classes IS 'Stocke les données des classes';

-- Compare this snippet from Bitbox/Backend/common/migrations/2024-06-26-130801_create_users_table/down.sql: