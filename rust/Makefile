#!make

fix: ## Fix rust code
	@cargo fix --allow-dirty --allow-staged --all-features --all-targets

fmt: ## Format rust code
	@cargo fmt --all -- --check

help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	awk 'BEGIN {FS = ":.*?## "}; \
	{printf "\033[36m%-25s\033[0m %s\n", $$1, $$2}' | \
	sort

lint: ## Lint rust code
	@cargo clippy --workspace --all-targets --verbose
	@cargo clippy --workspace --all-targets --verbose --all-features

tests: ## Run tests
	@cargo test --lib --verbose

.PHONY: fix fmt help tests lint

