CREATE TABLE "group_members" (
  "group_member_id" serial PRIMARY KEY,
  "group_id" int NOT NULL,
  "student_id" int NOT NULL,
  "created_at" timestamp DEFAULT (now())
);

CREATE INDEX ON "group_members" ("group_id");
CREATE INDEX ON "group_members" ("student_id");

COMMENT ON TABLE "group_members" IS 'Stocke les membres des groupes';
COMMENT ON COLUMN "group_members"."group_id" IS 'Identifiant du groupe';
COMMENT ON COLUMN "group_members"."student_id" IS 'Identifiant de l''élève';
COMMENT ON COLUMN "group_members"."created_at" IS 'Date de création';
