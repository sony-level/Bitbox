-- Your SQL goes here
CREATE TABLE IF NOT EXISTS group_users (
    group_id INT REFERENCES groups(id) ON DELETE CASCADE,
    user_id INT REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (group_id, user_id)
);

CREATE INDEX idx_group_users_group_id ON group_users(group_id);