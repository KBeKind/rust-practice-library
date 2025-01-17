
SHELL := /bin/bash
.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean

build: ## Build the project using cargo
	cargo build

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

fmt: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt

bump: ## Bump the version number
	@echo "Current version is $(shell grep "^version" Cargo.toml | cut -d\" -f2)"; \
	read -p "Enter new version number: " version; \
	sed -i -E "s/^version = \".*\"/version = \"$$version\"/" Cargo.toml; \
	echo "New version is $$(grep "^version" Cargo.toml | cut -d\" -f2)"
