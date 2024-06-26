-- Your SQL goes here
CREATE TABLE IF NOT EXISTS notifications (
    notification_id SERIAL PRIMARY KEY,
    recipient_id INT NOT NULL,
    sender_id INT NOT NULL,
    message TEXT NOT NULL,
    read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (recipient_id) REFERENCES users(user_id),
    FOREIGN KEY (sender_id) REFERENCES users(user_id)
);

CREATE INDEX IF NOT EXISTS notifications_recipient_id_idx ON notifications (recipient_id);
CREATE INDEX IF NOT EXISTS notifications_sender_id_idx ON notifications (sender_id);

COMMENT ON TABLE notifications IS 'Stocke les données des notifications';
COMMENT ON COLUMN notifications.recipient_id IS 'Identifiant du destinataire';
COMMENT ON COLUMN notifications.sender_id IS 'Identifiant de l''expéditeur';
COMMENT ON COLUMN notifications.message IS 'Message de la notification';
COMMENT ON COLUMN notifications.read IS 'Statut de lecture';
COMMENT ON COLUMN notifications.created_at IS 'Date de création';
