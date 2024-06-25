CREATE TABLE "reports" (
  "report_id" serial PRIMARY KEY,
  "class_id" int NOT NULL,
  "report_title" varchar(100) NOT NULL,
  "report_content" text NOT NULL,
  "created_at" timestamp DEFAULT (now()),
  "updated_at" timestamp DEFAULT (now())
);

CREATE INDEX ON "reports" ("class_id");

COMMENT ON TABLE "reports" IS 'Stocke les données des rapports';
COMMENT ON COLUMN "reports"."class_id" IS 'Identifiant de la classe';
COMMENT ON COLUMN "reports"."report_title" IS 'Titre du rapport';
COMMENT ON COLUMN "reports"."report_content" IS 'Contenu du rapport';
COMMENT ON COLUMN "reports"."created_at" IS 'Date de création';
COMMENT ON COLUMN "reports"."updated_at" IS 'Date de mise à jour';
