# Bitbox Project - PostgreSQL Deployment


## Prérequis

- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)


## Installation et utilisation

1. Clonez le dépôt et naviguez dans le répertoire `common` :

   ```bash
   git clone https://github.com/sony-level/Bitbox.git
   cd Bitbox/Backend/common


## Notes

- **Sécurisation :** Les informations sensibles comme les mots de passe sont stockées dans un fichier `.env` pour plus de sécurité.
- **Stockage Persistant :** Utilisation de PersistentVolumeClaim pour s'assurer que les données de la base de données sont conservées même si le pod est recréé.
