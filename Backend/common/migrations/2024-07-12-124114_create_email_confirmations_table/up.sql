-- Your SQL goes here

CREATE TABLE IF NOT EXISTS email_confirmations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL,
    token VARCHAR(255) NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP  DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP NOT NULL
    );

CREATE INDEX idx_email_confirmations_email ON email_confirmations(email);
CREATE INDEX idx_email_confirmations_token ON email_confirmations(token);
COMMENT ON TABLE email_confirmations IS 'Stocke les jetons de confirmation d''email';
COMMENT ON COLUMN email_confirmations.email IS 'L''adresse email';
COMMENT ON COLUMN email_confirmations.token IS 'Le jeton de confirmation d''email unique';
COMMENT ON COLUMN email_confirmations.expires_at IS 'Date d''expiration du jeton'