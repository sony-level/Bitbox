# Obtenir la liste des IDs de toutes les images sauf Kubernetes et Teleport
images_to_delete=$(docker images | grep -v 'k8s-minikube/kicbase\|public.ecr.aws/gravitational/teleport' | awk '{if(NR>1) print $3}')

# Supprimer les images filtrées
for image in $images_to_delete; do
    docker rmi -f $image
done
