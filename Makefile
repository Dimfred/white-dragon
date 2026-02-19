.ONESHELL:
SHELL := /bin/bash
.PHONY: build
all: help

################################################################################
# BUILD
build: ## build release binary
	cargo build --release

build-debug: ## build debug binary
	cargo build

clean: ## clean build artifacts
	cargo clean

################################################################################
# INSTALL
install: build ## install to ~/.local/bin
	mkdir -p ~/.local/bin
	cp target/release/white-dragon ~/.local/bin/
	@echo "Installed to ~/.local/bin/white-dragon"

uninstall: ## remove from ~/.local/bin
	rm -f ~/.local/bin/white-dragon
	@echo "Removed ~/.local/bin/white-dragon"

################################################################################
# RELEASE
github-release: build ## create GitHub release
	@VERSION=$$(grep -m1 '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	BINARY="releases/white-dragon-v$${VERSION}-macos-arm64"; \
	mkdir -p releases; \
	cp target/release/white-dragon "$$BINARY"; \
	echo "Binary copied to $$BINARY"; \
	if gh release view "v$$VERSION" >/dev/null 2>&1; then \
		echo "Release v$$VERSION already exists"; \
		read -p "Delete and recreate? [y/N] " -n 1 -r; \
		echo; \
		if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
			gh release delete "v$$VERSION" -y; \
		else \
			exit 1; \
		fi; \
	fi; \
	gh release create "v$$VERSION" "$$BINARY" \
		--title "v$$VERSION" \
		--notes "White Dragon v$$VERSION for macOS (Apple Silicon)" \
		--latest; \
	echo "Released v$$VERSION"

release: github-release ## create GitHub release and update brew formula
	@$(MAKE) brew-update
	@VERSION=$$(grep -m1 '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	git add Formula/white-dragon.rb; \
	git commit -m "chore: formula update"; \
	git push; \
	echo "Committed and pushed formula update"

version-patch: ## bump patch version (0.1.0 -> 0.1.1)
	@CURRENT=$$(grep -m1 '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	IFS='.' read -r MAJOR MINOR PATCH <<< "$$CURRENT"; \
	NEW_PATCH=$$((PATCH + 1)); \
	NEW_VERSION="$$MAJOR.$$MINOR.$$NEW_PATCH"; \
	sed -i '' "s/^version = \".*\"/version = \"$$NEW_VERSION\"/" Cargo.toml; \
	echo "Version bumped from $$CURRENT to $$NEW_VERSION"; \
	git add Cargo.toml; \
	git commit -m "chore: version bump"; \
	echo "Committed version bump"

################################################################################
# BREW
brew-update: ## update brew formula and push to homebrew-tap
	@VERSION=$$(grep -m1 '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	URL="https://github.com/Dimfred/white-dragon/archive/refs/tags/v$${VERSION}.tar.gz"; \
	echo "Fetching $$URL"; \
	SHA=$$(curl -sL "$$URL" | shasum -a 256 | cut -d' ' -f1); \
	echo "SHA256: $$SHA"; \
	sed -i '' "s|url \".*\"|url \"$$URL\"|" Formula/white-dragon.rb; \
	sed -i '' "s|sha256 \".*\"|sha256 \"$$SHA\"|" Formula/white-dragon.rb; \
	echo "Formula updated for v$$VERSION"; \
	cp Formula/white-dragon.rb ../homebrew-tap/Formula/; \
	cd ../homebrew-tap && git add . && git commit -m "Update white-dragon to v$$VERSION" && git push; \
	echo "Pushed to homebrew-tap"

################################################################################
# INIT
init: ## initialize development environment
	rustup default stable
	cargo fetch
	@echo "Development environment ready"

################################################################################
# HELP
help: ## print this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m%-20s\033[0m %s\n", $$1, $$2}'
