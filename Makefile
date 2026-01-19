.PHONY: all build test lint fmt dev clean run

all: fmt lint test build

build:
	cargo build

run:
	cargo run

dev:
	cargo run

test:
	cargo test

lint:
	cargo clippy -- -D warnings
	cargo fmt --all -- --check

fmt:
	cargo fmt --all

clean:
	cargo clean
