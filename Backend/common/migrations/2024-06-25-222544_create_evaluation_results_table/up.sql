-- Your SQL goes here
CREATE TABLE IF NOT EXISTS evaluation_results (
    evaluation_result_id SERIAL PRIMARY KEY,
    evaluation_id INT NOT NULL,
    student_id INT NOT NULL,
    score INT NOT NULL,
    feedback TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (evaluation_id) REFERENCES evaluations(evaluation_id),
    FOREIGN KEY (student_id) REFERENCES users(user_id)
);

CREATE INDEX IF NOT EXISTS evaluation_results_evaluation_id_idx ON evaluation_results (evaluation_id);
CREATE INDEX IF NOT EXISTS evaluation_results_student_id_idx ON evaluation_results (student_id);

COMMENT ON TABLE evaluation_results IS 'Stocke les résultats des évaluations';
COMMENT ON COLUMN evaluation_results.evaluation_id IS 'Identifiant de l''évaluation';
COMMENT ON COLUMN evaluation_results.student_id IS 'Identifiant de l''élève';
COMMENT ON COLUMN evaluation_results.score IS 'Score obtenu';
COMMENT ON COLUMN evaluation_results.feedback IS 'Retour d''évaluation';
COMMENT ON COLUMN evaluation_results.created_at IS 'Date de création';
COMMENT ON COLUMN evaluation_results.updated_at IS 'Date de mise à jour';
