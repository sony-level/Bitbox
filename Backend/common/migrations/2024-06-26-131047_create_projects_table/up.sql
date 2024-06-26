-- Your SQL goes here
CREATE TABLE IF NOT EXISTS projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_name VARCHAR(100) NOT NULL,
    class_id UUID REFERENCES classes(id) ON DELETE CASCADE,
    descriptions TEXT,
    start_date DATE,
    end_date DATE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_projects_name ON projects(project_name);
CREATE INDEX idx_projects_class_id ON projects(class_id);
COMMENT ON TABLE projects IS 'Stocke les projets de classes';
