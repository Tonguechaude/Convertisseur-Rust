# Création et initialisation d'un conteneur Docker

## Initialisation

### 1. Créer le Dockerfile


### 2. Fichiers source


### 3. Construire et exécuter le conteneur Docker

Ouvrez un terminal et naviguez jusqu'au répertoire contenant votre Dockerfile et main.c.

Construisez l'image Docker avec la commande suivante :

```sh
docker build -t convertisseur-rust-image:1.0.0 .
```

Exécutez le conteneur Docker avec la commande suivante :

```sh
    docker run --rm -it convertisseur-rust-image:1.0.0
```

Cette commande compile le programme à l'intérieur du conteneur Docker et exécute le binaire généré.
La sortie du programme sera affichée dans le terminal.
