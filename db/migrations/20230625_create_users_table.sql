CREATE TABLE "users" (
  "user_id" serial PRIMARY KEY,
  "username" varchar(50) UNIQUE NOT NULL,
  "password_hash" varchar(255) NOT NULL,
  "email" varchar(100) UNIQUE NOT NULL,
  "role" varchar(20) NOT NULL,
  "mfa_secret" varchar(255), -- Secret for MFA
  "created_at" timestamp DEFAULT (now()),
  "updated_at" timestamp DEFAULT (now())
);

COMMENT ON TABLE "users" IS 'Stocke les données des utilisateurs';
COMMENT ON COLUMN "users"."username" IS 'Nom d''utilisateur';
COMMENT ON COLUMN "users"."password_hash" IS 'Hash du mot de passe';
COMMENT ON COLUMN "users"."email" IS 'Adresse email';
COMMENT ON COLUMN "users"."role" IS 'Rôle de l''utilisateur (formateur, élève)';
COMMENT ON COLUMN "users"."mfa_secret" IS 'Secret pour l''authentification multi-facteurs';
COMMENT ON COLUMN "users"."created_at" IS 'Date de création';
COMMENT ON COLUMN "users"."updated_at" IS 'Date de mise à jour';
