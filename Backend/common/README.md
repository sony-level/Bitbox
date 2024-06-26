# Bitbox Project - PostgreSQL Deployment

## Instructions de Déploiement

1. **Construire l'image Docker :**

    ```bash
    docker build -t your-dockerhub-username/bitbox-postgres:latest .
    ```

2. **Pousser l'image Docker sur Docker Hub :**

    ```bash
    docker push your-dockerhub-username/bitbox-postgres:latest
    ```

3. **Déployer les ressources avec Docker Compose :**

    ```bash
    cd database
    docker-compose up -d
    ```

4. **Vérifier que le service PostgreSQL est en cours d'exécution :**

    ```bash
    docker ps
    ```

## Notes

- **Sécurisation :** Les informations sensibles comme les mots de passe sont stockées dans un fichier `.env` pour plus de sécurité.
- **Stockage Persistant :** Utilisation de PersistentVolumeClaim pour s'assurer que les données de la base de données sont conservées même si le pod est recréé.
