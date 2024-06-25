CREATE TABLE "groups" (
  "group_id" serial PRIMARY KEY,
  "group_name" varchar(100) NOT NULL,
  "class_id" int NOT NULL,
  "created_at" timestamp DEFAULT (now()),
  "updated_at" timestamp DEFAULT (now())
);

COMMENT ON TABLE "groups" IS 'Stocke les données des groupes';
COMMENT ON COLUMN "groups"."group_name" IS 'Nom du groupe';
COMMENT ON COLUMN "groups"."class_id" IS 'Identifiant de la classe';
COMMENT ON COLUMN "groups"."created_at" IS 'Date de création';
COMMENT ON COLUMN "groups"."updated_at" IS 'Date de mise à jour';

ALTER TABLE "groups" ADD FOREIGN KEY ("class_id") REFERENCES "classes" ("class_id");
