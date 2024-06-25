# Utiliser l'image officielle Teleport
FROM public.ecr.aws/gravitational/teleport:13.4.26

# Copier la configuration Teleport
COPY bitbox_teleport.yaml /etc/teleport.yaml

# Exposer les ports nécessaires pour Teleport
EXPOSE 3025 3080 3024 5433

# Commande pour démarrer Teleport avec la configuration spécifiée
CMD ["teleport", "start", "-c", "/etc/teleport.yaml"]
