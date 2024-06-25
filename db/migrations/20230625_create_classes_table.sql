CREATE TABLE "classes" (
  "class_id" serial PRIMARY KEY,
  "class_name" varchar(100) NOT NULL,
  "description" text,
  "teacher_id" int NOT NULL,
  "created_at" timestamp DEFAULT (now()),
  "updated_at" timestamp DEFAULT (now())
);

COMMENT ON TABLE "classes" IS 'Stocke les données des classes';
COMMENT ON COLUMN "classes"."class_name" IS 'Nom de la classe';
COMMENT ON COLUMN "classes"."description" IS 'Description de la classe';
COMMENT ON COLUMN "classes"."teacher_id" IS 'Identifiant du formateur';
COMMENT ON COLUMN "classes"."created_at" IS 'Date de création';
COMMENT ON COLUMN "classes"."updated_at" IS 'Date de mise à jour';

ALTER TABLE "classes" ADD FOREIGN KEY ("teacher_id") REFERENCES "users" ("user_id");
