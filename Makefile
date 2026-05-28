# =============================================================================
# quire-wasm Makefile
# =============================================================================

CARGO ?= cargo
WASM_PACK ?= wasm-pack

# Test parity uses the in-repo spec-artifacts-iso module sibling.
QUIRE_WASM_TEST_MODULE_ROOT ?= $(abspath ../spec-artifacts-iso/spec_artifacts_iso)

.PHONY: help
help:
	@echo "Available targets:"
	@echo "  make fmt              - Format with rustfmt"
	@echo "  make fmt-check        - Verify formatting (CI gate)"
	@echo "  make lint             - Clippy with -D warnings"
	@echo "  make test             - wasm-pack test --node (parity tests)"
	@echo "  make build            - wasm-pack build --target web --release"
	@echo "  make build-node       - wasm-pack build --target nodejs --release"
	@echo "  make clean            - cargo clean + rm -rf pkg/"
	@echo "  make deny             - cargo deny check licenses"
	@echo "  make audit-unsafe     - Enforce // SAFETY: comments on unsafe blocks"
	@echo "  make pack             - npm pack (produces .tgz tarball)"
	@echo "  make ci               - fmt-check + lint + test + deny + audit-unsafe"

.PHONY: fmt
fmt:
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check:
	$(CARGO) fmt --all -- --check

.PHONY: lint
lint:
	$(CARGO) clippy --all-targets --target wasm32-unknown-unknown -- -D warnings

.PHONY: test
test:
	QUIRE_WASM_TEST_MODULE_ROOT=$(QUIRE_WASM_TEST_MODULE_ROOT) $(WASM_PACK) test --node

.PHONY: build
build:
	$(WASM_PACK) build --target web --release

.PHONY: build-node
build-node:
	$(WASM_PACK) build --target nodejs --release

.PHONY: clean
clean:
	$(CARGO) clean
	rm -rf pkg/

.PHONY: deny
deny:
	$(CARGO) deny check licenses

.PHONY: audit-unsafe
audit-unsafe:
	bash scripts/check_unsafe_comments.sh

.PHONY: pack
pack: build
	cd pkg && npm pack

.PHONY: ci
ci: fmt-check lint test deny audit-unsafe
