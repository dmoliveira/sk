PROJECT := sk
VERSION := 0.2.1
TAG ?= v$(VERSION)

.PHONY: help fmt test build smoke docs-guard ci release-snippet

help: ## Show commands
	@printf "%s %s\n" "$(PROJECT)" "v$(VERSION)"
	@printf "%-12s %s\n" "help" "Show commands"
	@printf "%-12s %s\n" "fmt" "Run rustfmt"
	@printf "%-12s %s\n" "test" "Run unit tests"
	@printf "%-12s %s\n" "build" "Build release binary"
	@printf "%-12s %s\n" "smoke" "Run keychain smoke test"
	@printf "%-12s %s\n" "docs-guard" "Check docs trust links"
	@printf "%-12s %s\n" "ci" "Run local CI checks"
	@printf "%-12s %s\n" "release-snippet" "Print tap URL/SHA snippet"

fmt: ## Run rustfmt
	cargo fmt

test: ## Run unit tests
	cargo test

build: ## Build release binary
	cargo build --release

smoke: ## Run keychain smoke test
	./scripts/smoke.sh

docs-guard: ## Check docs trust links
	./scripts/check-doc-links.sh

ci: ## Run local CI checks
	cargo fmt --check
	cargo test
	./scripts/check-doc-links.sh

release-snippet: ## Print tap URL/SHA snippet
	./scripts/release-tap-snippet.sh "$(TAG)"
