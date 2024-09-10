cargo build --release \
&& cargo test \
&& docker build -t convertisseur_rust . \
&& docker run --rm -it convertisseur_rust