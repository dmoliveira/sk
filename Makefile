PROJECT := sk
VERSION := 0.2.0

.PHONY: help fmt test build smoke ci

help: ## Show commands
	@printf "%s %s\n" "$(PROJECT)" "v$(VERSION)"
	@printf "%-12s %s\n" "help" "Show commands"
	@printf "%-12s %s\n" "fmt" "Run rustfmt"
	@printf "%-12s %s\n" "test" "Run unit tests"
	@printf "%-12s %s\n" "build" "Build release binary"
	@printf "%-12s %s\n" "smoke" "Run keychain smoke test"
	@printf "%-12s %s\n" "ci" "Run local CI checks"

fmt: ## Run rustfmt
	cargo fmt

test: ## Run unit tests
	cargo test

build: ## Build release binary
	cargo build --release

smoke: ## Run keychain smoke test
	./scripts/smoke.sh

ci: ## Run local CI checks
	cargo fmt --check
	cargo test
