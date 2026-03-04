.PHONY: build test lint lint-md lint-all fmt ci hooks run clean

build:
	cargo build --release

test:
	cargo test

lint:
	cargo clippy --all-targets -- -D warnings

lint-md:
	npx --yes markdownlint-cli2 "**/*.md" "#target" "#node_modules" "#*/node_modules" "#.git"

lint-all: lint lint-md

fmt:
	cargo fmt --all -- --check

ci: fmt lint-all test build

hooks:
	cp scripts/pre-push .git/hooks/pre-push

run:
	cargo run

clean:
	cargo clean
