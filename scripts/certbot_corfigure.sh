#!/bin/bash

# Fonction pour afficher le logo
afficher_logo() {
    printf "\n"
    printf "  _____       _                   _     \n"
    printf " |_   _|     (_)                 | |    \n"
    printf "   | |  _ __  _  ___  _ __   __ _| |__  \n"
    printf "   | | | '_ \\| |/ _ \\| '_ \\ / _\` | '_ \\ \n"
    printf "  _| |_| | | | | (_) | | | | (_| | | | |\n"
    printf " |_____|_| |_|_|\\___/|_| |_|\\__, |_| |_|\n"
    printf "                              __/ |     \n"
    printf "                             |___/      \n"
    printf "\n"
    printf "             Script par Sony\n"
    printf "\n"
}

# Fonction pour installer Certbot
installer_certbot() {
    local distro
    distro=$(grep ^ID= /etc/os-release | cut -d= -f2 | tr -d '"')
    
    if [[ "$distro" == "ubuntu" || "$distro" == "debian" || "$distro" == "kali" ]]; then
        sudo apt-get update
        sudo apt-get install -y certbot
    elif [[ "$distro" == "centos" || "$distro" == "rhel" ]]; then
        sudo yum install -y epel-release
        sudo yum install -y certbot
    else
        printf "Distribution non prise en charge. Veuillez installer Certbot manuellement.\n" >&2
        return 1
    fi
}

# Fonction pour obtenir le certificat Let's Encrypt
obtenir_certificat_lets_encrypt() {
    local serveur_web
    read -p "Entrez votre serveur web (apache/nginx/aucun): " serveur_web

    if [[ "$serveur_web" == "apache" ]]; then
        sudo certbot --apache
    elif [[ "$serveur_web" == "nginx" ]]; then
        sudo certbot --nginx
    elif [[ "$serveur_web" == "aucun" ]]; then
        sudo certbot certonly --standalone
    else
        printf "Choix de serveur web invalide.\n" >&2
        return 1
    fi
}

# Fonction pour générer un certificat auto-signé
generer_certificat_auto_signe() {
    local chemin_cle chemin_cert
    read -p "Entrez le chemin pour sauvegarder votre clé: " chemin_cle
    read -p "Entrez le chemin pour sauvegarder votre certificat: " chemin_cert

    openssl req -x509 -newkey rsa:4096 -keyout "$chemin_cle" -out "$chemin_cert" -days 365
}

# Fonction pour mettre à jour le fichier de configuration de Teleport
mettre_a_jour_config_teleport() {
    local chemin_cle chemin_cert chemin_config
    read -p "Entrez le chemin vers votre fichier de configuration Teleport: " chemin_config
    read -p "Entrez le chemin vers votre fichier de clé: " chemin_cle
    read -p "Entrez le chemin vers votre fichier de certificat: " chemin_cert

    if [[ ! -f "$chemin_config" || ! -f "$chemin_cle" || ! -f "$chemin_cert" ]]; then
        printf "Le fichier de configuration ou les fichiers de clés/certificats n'existent pas. Veuillez fournir des chemins valides.\n" >&2
        return 1
    fi

    sed -i "s|https_keypairs: \[\]|https_keypairs:\n    - key_file: $chemin_cle\n      cert_file: $chemin_cert|g" "$chemin_config"

    printf "Mise à jour de %s avec les chemins de clé et de certificat fournis.\n" "$chemin_config"
}

# Fonction principale
Main() {
    afficher_logo

    printf "Choisissez une option:\n"
    printf "1. Installer Certbot et obtenir un certificat Let's Encrypt\n"
    printf "2. Générer un certificat auto-signé\n"
    read -p "Entrez votre choix (1/2): " choix

    case "$choix" in
        1)
            if ! installer_certbot; then
                printf "Échec de l'installation de Certbot.\n" >&2
                return 1
            fi
            if ! obtenir_certificat_lets_encrypt; then
                printf "Échec de l'obtention du certificat Let's Encrypt.\n" >&2
                return 1
            fi
            ;;
        2)
            if ! generer_certificat_auto_signe; then
                printf "Échec de la génération du certificat auto-signé.\n" >&2
                return 1
            fi
            ;;
        *)
            printf "Choix invalide. Abandon.\n" >&2
            return 1
            ;;
    esac

    if ! mettre_a_jour_config_teleport; then
        printf "Échec de la mise à jour du fichier de configuration Teleport.\n" >&2
        return 1
    fi

    printf "Tâche terminée avec succès.\n"
}

# Exécuter la fonction principale
Main
