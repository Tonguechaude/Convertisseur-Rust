# Makefile for convertisseur-rust

test:
	cargo test -- --nocapture
.PHONY: run

lint:
	cargo fmt -- --check
.PHONY: run

build:
	cargo build --release
.PHONY: run

run:
	cargo run --release
.PHONY: run

clean:
	cargo clean
.PHONY: clean
