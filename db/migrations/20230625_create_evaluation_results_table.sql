CREATE TABLE "evaluation_results" (
  "evaluation_result_id" serial PRIMARY KEY,
  "evaluation_id" int NOT NULL,
  "student_id" int NOT NULL,
  "score" int NOT NULL,
  "feedback" text,
  "created_at" timestamp DEFAULT (now()),
  "updated_at" timestamp DEFAULT (now())
);

CREATE INDEX ON "evaluation_results" ("evaluation_id");
CREATE INDEX ON "evaluation_results" ("student_id");

COMMENT ON TABLE "evaluation_results" IS 'Stocke les résultats des évaluations';
COMMENT ON COLUMN "evaluation_results"."evaluation_id" IS 'Identifiant de l''évaluation';
COMMENT ON COLUMN "evaluation_results"."student_id" IS 'Identifiant de l''élève';
COMMENT ON COLUMN "evaluation_results"."score" IS 'Score obtenu';
COMMENT ON COLUMN "evaluation_results"."feedback" IS 'Retour d''évaluation';
COMMENT ON COLUMN "evaluation_results"."created_at" IS 'Date de création';
COMMENT ON COLUMN "evaluation_results"."updated_at" IS 'Date de mise à jour';
