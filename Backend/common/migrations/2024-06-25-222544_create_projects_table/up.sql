-- Your SQL goes here

CREATE TABLE IF NOT EXISTS projects (
    project_id SERIAL PRIMARY KEY,
    project_name VARCHAR(100) NOT NULL,
    description TEXT,
    class_id INT NOT NULL,
    leader_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (class_id) REFERENCES classes(class_id),
    FOREIGN KEY (leader_id) REFERENCES users(user_id)
);

CREATE INDEX IF NOT EXISTS projects_class_id_idx ON projects (class_id);
CREATE INDEX IF NOT EXISTS projects_leader_id_idx ON projects (leader_id);

COMMENT ON TABLE projects IS 'Stocke les données des projets';
COMMENT ON COLUMN projects.project_name IS 'Nom du projet';
COMMENT ON COLUMN projects.description IS 'Description du projet';
COMMENT ON COLUMN projects.class_id IS 'Identifiant de la classe';
COMMENT ON COLUMN projects.leader_id IS 'Identifiant du responsable du projet';
COMMENT ON COLUMN projects.created_at IS 'Date de création';
COMMENT ON COLUMN projects.updated_at IS 'Date de mise à jour';
