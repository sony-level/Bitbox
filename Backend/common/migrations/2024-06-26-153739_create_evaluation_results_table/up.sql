-- Your SQL goes here
CREATE TABLE IF NOT EXISTS evaluation_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    group_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    average_score FLOAT NOT NULL,
    final_score FLOAT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_evaluation_results_user_id ON evaluation_results(user_id);
CREATE INDEX idx_evaluation_results_group_id ON evaluation_results(group_id);

