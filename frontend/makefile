TRUNK_VERSION:=v0.16.0
.PHONY: help deps deps-pnpm deps-builder-install deps-builder-pre-built serve build clean
help:
	@echo "Usage: make [target]"
	@echo "  help"
	@echo "  deps                    - install all dependencies"
	@echo "  deps-pnpm"
	@echo "  deps-builder-install	 - install trunk from source"
	@echo "  deps-builder-pre-built  - install pre-built trunk binary"
	@echo "  serve"
	@echo "  build                   - build for releaes"
	@echo "  build-in-docker 		 - build release in docker"
	@echo "  clean                   - clean build artifacts"
deps: deps-pnpm deps-builder-pre-built
deps-pnpm:
	@pnpm i
deps-builder-install:
	@cargo install --locked trunk
deps-builder-pre-built:
	wget -qO- https://github.com/thedodd/trunk/releases/download/${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
	chmod +x trunk
serve:
	@trunk serve
build:
	@trunk build --release
build-in-docker:
	@docker pull ghcr.io/enchant97/trunk-docker-builder:latest
	@docker run --rm -v $(shell pwd):/app ghcr.io/enchant97/trunk-docker-builder
clean:
	@trunk clean
