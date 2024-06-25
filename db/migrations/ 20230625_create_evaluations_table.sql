CREATE TABLE "evaluations" (
  "evaluation_id" serial PRIMARY KEY,
  "class_id" int NOT NULL,
  "title" varchar(100) NOT NULL,
  "description" text,
  "max_score" int NOT NULL,
  "created_at" timestamp DEFAULT (now()),
  "updated_at" timestamp DEFAULT (now())
);

CREATE INDEX ON "evaluations" ("class_id");

COMMENT ON TABLE "evaluations" IS 'Stocke les données des évaluations';
COMMENT ON COLUMN "evaluations"."class_id" IS 'Identifiant de la classe';
COMMENT ON COLUMN "evaluations"."title" IS 'Titre de l''évaluation';
COMMENT ON COLUMN "evaluations"."description" IS 'Description de l''évaluation';
COMMENT ON COLUMN "evaluations"."max_score" IS 'Score maximal';
COMMENT ON COLUMN "evaluations"."created_at" IS 'Date de création';
COMMENT ON COLUMN "evaluations"."updated_at" IS 'Date de mise à jour';

ALTER TABLE "evaluations" ADD FOREIGN KEY ("class_id") REFERENCES "classes" ("class_id");
