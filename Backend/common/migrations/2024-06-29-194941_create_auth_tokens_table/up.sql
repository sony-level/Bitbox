-- Your SQL goes here
CREATE TABLE IF NOT EXISTS auth_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    token VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL
);

COMMENT ON TABLE auth_tokens IS 'Stocke les jetons d''authentification';
COMMENT ON COLUMN auth_tokens.token IS 'Le jeton d''authentification unique';
COMMENT ON COLUMN auth_tokens.expires_at IS 'Date d''expiration du jeton';
