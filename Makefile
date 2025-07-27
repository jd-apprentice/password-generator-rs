app_name ?= password-generator

all: test lint format build

start:
	./target/release/

dev:
	cargo watch -x run

build: src/main.rs
	cargo build --release --bin password-generator

test:
	cargo test

lint:
	cargo clippy --all-targets --all-features

lint-fix:
	cargo clippy --all-targets --all-features --fix --allow-dirty

format:
	cargo fmt --all --check

format-fix:
	cargo fmt --all

.PHONY: build test lint format
