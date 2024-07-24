#!/bin/bash

set -euo pipefail

# variables globales de l'installentation
OS=$(uname -s)
ARCH=$(uname -m)
MINIKUBE_VERSION="latest"
MINIKUBE_URL_BASE="https://storage.googleapis.com/minikube/releases/${MINIKUBE_VERSION}"

# Fonction permetant d'afficher le logo
# Kubernetes ASCII Logo
print_logo() {
    printf "\n"
    printf " _          _                         _           \n"
    printf "| | ___   _| | _____ _ __  _   _ _ __(_) ___  ___ \n"
    printf "| |/ / | | | |/ / _ \ '_ \| | | | '__| |/ _ \/ __|\n"
    printf "|   <| |_| |   <  __/ | | | |_| | |  | |  __/\__ \\\n"
    printf "|_|\_\\__,_|_|\_\___|_| |_|\__,_|_|  |_|\___||___/\n"
    printf "\n"
    printf "                  by Level Sony\n"
    printf "\n"
}

# fonction permetant d'installer les dependances
install_dependencies() {
    case "$OS" in
        Linux)
            sudo apt-get update -y
            sudo apt-get install -y curl apt-transport-https ca-certificates software-properties-common
            ;;
        Darwin)
            if ! command -v brew >/dev/null; then
                printf "Homebrew is not installed. Installing Homebrew...\n"
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            fi
            ;;
        *)
            printf "Unsupported OS: %s\n" "$OS" >&2
            return 1
            ;;
    esac
}

# Fonction permetenat d'installa docker
install_docker() {
    if ! command -v docker >/dev/null; then
        case "$OS" in
            Linux)
                curl -fsSL https://get.docker.com | sh
                sudo usermod -aG docker "$USER"
                newgrp docker
                ;;
            Darwin)
                brew install --cask docker
                open /Applications/Docker.app
                printf "Please follow the Docker app instructions to complete the setup.\n"
                ;;
            *)
                printf "Unsupported OS: %s\n" "$OS" >&2
                return 1
                ;;
        esac
    else
        printf "Docker is already installed.\n"
    fi
}

# Function to install Minikube
install_minikube() {
    case "$OS" in
        Linux)
            case "$ARCH" in
                x86_64)
                    if command -v dpkg >/dev/null; then
                        curl -LO ${MINIKUBE_URL_BASE}/minikube_latest_amd64.deb
                        sudo dpkg -i minikube_latest_amd64.deb
                        rm minikube_latest_amd64.deb
                    elif command -v rpm >/dev/null; then
                        curl -LO ${MINIKUBE_URL_BASE}/minikube-latest.x86_64.rpm
                        sudo rpm -Uvh minikube-latest.x86_64.rpm
                        rm minikube-latest.x86_64.rpm
                    else
                        curl -LO ${MINIKUBE_URL_BASE}/minikube-linux-amd64
                        sudo install minikube-linux-amd64 /usr/local/bin/minikube
                        rm minikube-linux-amd64
                    fi
                    ;;
                *)
                    printf "Unsupported architecture: %s\n" "$ARCH" >&2
                    return 1
                    ;;
            esac
            ;;
        Darwin)
            case "$ARCH" in
                x86_64)
                    curl -LO ${MINIKUBE_URL_BASE}/minikube-darwin-amd64
                    sudo install minikube-darwin-amd64 /usr/local/bin/minikube
                    rm minikube-darwin-amd64
                    ;;
                arm64)
                    curl -LO ${MINIKUBE_URL_BASE}/minikube-darwin-arm64
                    sudo install minikube-darwin-arm64 /usr/local/bin/minikube
                    rm minikube-darwin-arm64
                    ;;
                *)
                    printf "Unsupported architecture: %s\n" "$ARCH" >&2
                    return 1
                    ;;
            esac
            ;;
        *)
            printf "Unsupported OS: %s\n" "$OS" >&2
            return 1
            ;;
    esac
}

# lancer  minikube
start_minikube() {
    if ! minikube start; then
        printf "Minikube failed to start. Please check the error messages above.\n" >&2
        return 1
    fi
    printf "Minikube started successfully.\n"
}

# Fonction principale
main() {
    print_logo
    install_dependencies
    install_docker
    install_minikube
    start_minikube
}

main
