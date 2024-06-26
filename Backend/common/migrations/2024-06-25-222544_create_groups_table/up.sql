-- Your SQL goes here
CREATE TABLE IF NOT EXISTS groups (
    group_id SERIAL PRIMARY KEY,
    group_name VARCHAR(100) NOT NULL,
    class_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (class_id) REFERENCES classes(class_id)
);

CREATE INDEX IF NOT EXISTS groups_class_id_idx ON groups (class_id);

COMMENT ON TABLE groups IS 'Stocke les données des groupes';
COMMENT ON COLUMN groups.group_name IS 'Nom du groupe';
COMMENT ON COLUMN groups.class_id IS 'Identifiant de la classe';
COMMENT ON COLUMN groups.created_at IS 'Date de création';
COMMENT ON COLUMN groups.updated_at IS 'Date de mise à jour';
