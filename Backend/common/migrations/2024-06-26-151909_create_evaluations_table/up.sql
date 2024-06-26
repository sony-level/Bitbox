-- Your SQL goes here
CREATE TABLE IF NOT EXISTS evaluations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    evaluator_id UUID REFERENCES users(id) ON DELETE CASCADE,
    evaluatee_id UUID REFERENCES users(id) ON DELETE CASCADE,
    group_id UUID REFERENCES groups(id) ON DELETE CASCADE,
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    score INT NOT NULL,
    comments TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_evaluation UNIQUE (evaluator_id, evaluatee_id, group_id, project_id)
);


CREATE INDEX idx_evaluations_evaluator_id ON evaluations(evaluator_id);
CREATE INDEX idx_evaluations_evaluatee_id ON evaluations(evaluatee_id);
CREATE INDEX idx_evaluations_group_id ON evaluations(group_id);
CREATE INDEX idx_evaluations_project_id ON evaluations(project_id);