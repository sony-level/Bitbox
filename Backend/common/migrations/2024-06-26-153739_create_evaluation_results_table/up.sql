-- Your SQL goes here
CREATE TABLE IF NOT EXISTS evaluation_results (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id) ON DELETE CASCADE,
    group_id INT REFERENCES groups(id) ON DELETE CASCADE,
    project_id INT REFERENCES projects(id) ON DELETE CASCADE,
    average_score FLOAT NOT NULL,
    final_score FLOAT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_evaluation_results_user_id ON evaluation_results(user_id);
CREATE INDEX idx_evaluation_results_group_id ON evaluation_results(group_id);
CREATE INDEX idx_evaluation_results_project_id ON evaluation_results(project_id);
