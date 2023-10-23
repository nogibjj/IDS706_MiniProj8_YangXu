install:
	cargo build

test:
	cargo test

format:
	cargo fmt

lint:
	cargo clippy

all: install lint format test

setup_package:
	cargo install --path .

