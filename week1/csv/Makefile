.PHONY: all clean format lint test run release

clean:
	cargo clean

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

build:
	cargo build

release:
	cargo build --release

all: format lint test release
