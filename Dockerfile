FROM ubuntu:24.04

# Installer bash pour un shell interactif
RUN apt-get update && apt-get install --no-install-recommends -y bash \
&& apt-get clean \
&& rm -rf /var/lib/apt/lists/*

# Créer un dossier de travail
WORKDIR /usr/src/convertisseur

# Copier le binaire compilé depuis l'environnement CI
COPY . /usr/src/convertisseur

# Définir le point d'entrée par défaut
CMD ["./target/release/convertisseur_rust"]
