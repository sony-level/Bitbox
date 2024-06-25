CREATE TABLE "notifications" (
  "notification_id" serial PRIMARY KEY,
  "recipient_id" int NOT NULL,
  "sender_id" int NOT NULL,
  "message" text NOT NULL,
  "read" boolean DEFAULT false,
  "created_at" timestamp DEFAULT (now())
);

CREATE INDEX ON "notifications" ("recipient_id");
CREATE INDEX ON "notifications" ("sender_id");

COMMENT ON TABLE "notifications" IS 'Stocke les données des notifications';
COMMENT ON COLUMN "notifications"."recipient_id" IS 'Identifiant du destinataire';
COMMENT ON COLUMN "notifications"."sender_id" IS 'Identifiant de l''expéditeur';
COMMENT ON COLUMN "notifications"."message" IS 'Message de la notification';
COMMENT ON COLUMN "notifications"."read" IS 'Statut de lecture';
COMMENT ON COLUMN "notifications"."created_at" IS 'Date de création';
