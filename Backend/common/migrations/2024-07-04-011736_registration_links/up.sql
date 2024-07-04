-- Your SQL goes here
CREATE TABLE IF NOT EXISTS registration_links (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    token VARCHAR(255) UNIQUE NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    used_at TIMESTAMP,
    used_by UUID REFERENCES users(id) ON DELETE CASCADE,
    project_id UUID REFERENCES projects(id) ON DELETE CASCADE,
    group_id UUID REFERENCES groups(id) ON DELETE CASCADE,
    class_id UUID REFERENCES classes(id) ON DELETE CASCADE,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    role user_role NOT NULL,
    email VARCHAR(255) NOT NULL,
    link TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_registration_links_user_id ON registration_links(user_id);
CREATE INDEX idx_registration_links_project_id ON registration_links(project_id);
CREATE INDEX idx_registration_links_group_id ON registration_links(group_id);
CREATE INDEX idx_registration_links_class_id ON registration_links(class_id);
CREATE INDEX idx_registration_links_used_by ON registration_links(used_by);