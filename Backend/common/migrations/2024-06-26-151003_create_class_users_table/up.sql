-- Your SQL goes here
CREATE TABLE IF NOT EXISTS class_users (
    class_id INT REFERENCES classes(id) ON DELETE CASCADE,
    user_id INT REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (class_id, user_id)
);

CREATE INDEX idx_class_users_class_id ON class_users(class_id);
CREATE INDEX idx_class_users_user_id ON class_users(user_id);
