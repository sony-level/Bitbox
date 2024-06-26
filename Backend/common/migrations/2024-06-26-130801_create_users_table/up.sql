-- Your SQL goes here

CREATE TABLE IF NOT EXISTS users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    role VARCHAR(20) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

COMMENT ON TABLE users IS 'Stocke les données des utilisateurs';
COMMENT ON COLUMN users.username IS 'Nom d''utilisateur';
COMMENT ON COLUMN users.password_hash IS 'Hash du mot de passe';
COMMENT ON COLUMN users.email IS 'Adresse email';
COMMENT ON COLUMN users.role IS 'Rôle de l''utilisateur (formateur, élève)';
COMMENT ON COLUMN users.created_at IS 'Date de création';
COMMENT ON COLUMN users.updated_at IS 'Date de mise à jour';
