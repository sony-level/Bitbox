CREATE TABLE "projects" (
  "project_id" serial PRIMARY KEY,
  "project_name" varchar(100) NOT NULL,
  "description" text,
  "class_id" int NOT NULL,
  "leader_id" int NOT NULL,
  "created_at" timestamp DEFAULT (now()),
  "updated_at" timestamp DEFAULT (now())
);

CREATE INDEX ON "projects" ("class_id");
CREATE INDEX ON "projects" ("leader_id");

COMMENT ON TABLE "projects" IS 'Stocke les données des projets';
COMMENT ON COLUMN "projects"."project_name" IS 'Nom du projet';
COMMENT ON COLUMN "projects"."description" IS 'Description du projet';
COMMENT ON COLUMN "projects"."class_id" IS 'Identifiant de la classe';
COMMENT ON COLUMN "projects"."leader_id" IS 'Identifiant du responsable du projet';
COMMENT ON COLUMN "projects"."created_at" IS 'Date de création';
COMMENT ON COLUMN "projects"."updated_at" IS 'Date de mise à jour';
