# Incinerator Sentinel — objetivos de desarrollo y release.

CARGO ?= cargo
TARGET ?= x86_64-unknown-linux-musl
FEATURES ?= hardened,volatile-audit

.PHONY: help fmt clippy test check build release deny audit clean

help:
	@echo "fmt      - comprobar formato"
	@echo "clippy   - lints estrictos"
	@echo "test     - tests"
	@echo "check    - fmt + clippy + test"
	@echo "build    - build release"
	@echo "release  - build reproducible + auditoria"

fmt:
	$(CARGO) fmt --all --check

clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

test:
	$(CARGO) test --all

check: fmt clippy test

build:
	$(CARGO) build --release --locked

release:
	SOURCE_DATE_EPOCH=$$(git log -1 --pretty=%ct 2>/dev/null || date +%s) \
	$(CARGO) build --release --locked --target $(TARGET) \
		--no-default-features --features $(FEATURES)
	$(CARGO) auditable

deny:
	$(CARGO) deny check

audit:
	$(CARGO) audit

clean:
	$(CARGO) clean
