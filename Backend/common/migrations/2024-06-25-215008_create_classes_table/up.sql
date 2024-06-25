-- Your SQL goes here
CREATE TABLE IF NOT EXISTS classes (
    class_id SERIAL PRIMARY KEY,
    class_name VARCHAR(100) NOT NULL,
    description TEXT,
    teacher_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (teacher_id) REFERENCES users(user_id)
);

CREATE INDEX ON classes (teacher_id);

COMMENT ON TABLE classes IS 'Stocke les données des classes';
COMMENT ON COLUMN classes.class_name IS 'Nom de la classe';
COMMENT ON COLUMN classes.description IS 'Description de la classe';
COMMENT ON COLUMN classes.teacher_id IS 'Identifiant du formateur';
COMMENT ON COLUMN classes.created_at IS 'Date de création';
COMMENT ON COLUMN classes.updated_at IS 'Date de mise à jour';
