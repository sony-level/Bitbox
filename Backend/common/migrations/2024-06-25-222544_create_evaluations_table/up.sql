-- Your SQL goes here
CREATE TABLE IF NOT EXISTS evaluations (
    evaluation_id SERIAL PRIMARY KEY,
    class_id INT NOT NULL,
    title VARCHAR(100) NOT NULL,
    description TEXT,
    max_score INT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (class_id) REFERENCES classes(class_id)
);

CREATE INDEX IF NOT EXISTS evaluations_class_id_idx ON evaluations (class_id);

COMMENT ON TABLE evaluations IS 'Stocke les données des évaluations';
COMMENT ON COLUMN evaluations.class_id IS 'Identifiant de la classe';
COMMENT ON COLUMN evaluations.title IS 'Titre de l''évaluation';
COMMENT ON COLUMN evaluations.description IS 'Description de l''évaluation';
COMMENT ON COLUMN evaluations.max_score IS 'Score maximal';
COMMENT ON COLUMN evaluations.created_at IS 'Date de création';
COMMENT ON COLUMN evaluations.updated_at IS 'Date de mise à jour';
