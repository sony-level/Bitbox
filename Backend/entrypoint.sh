#!/bin/bash
set -e

# Charger les variables d'environnement depuis le fichier .env
export $(cat .env | xargs)

# Attendre que PostgreSQL soit prêt
until pg_isready -h db -p 5432 -U "$POSTGRES_USER"; do
  echo "Waiting for postgres container..."
  sleep 2
done

# Exécuter les migrations Diesel
diesel migration run

exec "$@"
