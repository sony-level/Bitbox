-- Your SQL goes here
CREATE TABLE IF NOT EXISTS group_members (
    group_member_id SERIAL PRIMARY KEY,
    group_id INT NOT NULL,
    student_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (group_id) REFERENCES groups(group_id),
    FOREIGN KEY (student_id) REFERENCES users(user_id)
);

CREATE INDEX IF NOT EXISTS group_members_group_id_idx ON group_members (group_id);
CREATE INDEX IF NOT EXISTS group_members_student_id_idx ON group_members (student_id);

COMMENT ON TABLE group_members IS 'Stocke les membres des groupes';
COMMENT ON COLUMN group_members.group_id IS 'Identifiant du groupe';
COMMENT ON COLUMN group_members.student_id IS 'Identifiant de l''élève';
COMMENT ON COLUMN group_members.created_at IS 'Date de création';
