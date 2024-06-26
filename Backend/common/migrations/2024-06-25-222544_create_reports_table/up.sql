-- Your SQL goes here
CREATE TABLE IF NOT EXISTS reports (
    report_id SERIAL PRIMARY KEY,
    class_id INT NOT NULL,
    report_title VARCHAR(100) NOT NULL,
    report_content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (class_id) REFERENCES classes(class_id)
);

CREATE INDEX IF NOT EXISTS reports_class_id_idx ON reports (class_id);

COMMENT ON TABLE reports IS 'Stocke les données des rapports';
COMMENT ON COLUMN reports.class_id IS 'Identifiant de la classe';
COMMENT ON COLUMN reports.report_title IS 'Titre du rapport';
COMMENT ON COLUMN reports.report_content IS 'Contenu du rapport';
COMMENT ON COLUMN reports.created_at IS 'Date de création';
COMMENT ON COLUMN reports.updated_at IS 'Date de mise à jour';
