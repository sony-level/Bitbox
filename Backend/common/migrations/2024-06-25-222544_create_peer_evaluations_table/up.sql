-- Your SQL goes here
CREATE TABLE IF NOT EXISTS peer_evaluations (
    peer_evaluation_id SERIAL PRIMARY KEY,
    evaluation_id INT NOT NULL,
    evaluator_id INT NOT NULL,
    evaluatee_id INT NOT NULL,
    score INT NOT NULL,
    feedback TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (evaluation_id) REFERENCES evaluations(evaluation_id),
    FOREIGN KEY (evaluator_id) REFERENCES users(user_id),
    FOREIGN KEY (evaluatee_id) REFERENCES users(user_id)
);

CREATE INDEX IF NOT EXISTS peer_evaluations_evaluation_id_idx ON peer_evaluations (evaluation_id);
CREATE INDEX IF NOT EXISTS peer_evaluations_evaluator_id_idx ON peer_evaluations (evaluator_id);
CREATE INDEX IF NOT EXISTS peer_evaluations_evaluatee_id_idx ON peer_evaluations (evaluatee_id);

COMMENT ON TABLE peer_evaluations IS 'Stocke les évaluations 360';
COMMENT ON COLUMN peer_evaluations.evaluation_id IS 'Identifiant de l\'évaluation';
COMMENT ON COLUMN peer_evaluations.evaluator_id IS 'Identifiant de l\'évaluateur';
COMMENT ON COLUMN peer_evaluations.evaluatee_id IS 'Identifiant de l\'évalué';
COMMENT ON COLUMN peer_evaluations.score IS 'Score attribué';
COMMENT ON COLUMN peer_evaluations.feedback IS 'Retour d\'évaluation';
COMMENT ON COLUMN peer_evaluations.created_at IS 'Date de création';
COMMENT ON COLUMN peer_evaluations.updated_at IS 'Date de mise à jour';
