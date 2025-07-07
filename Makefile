CLIPPY=cargo clippy --release --fix --allow-dirty --allow-staged --features="helpers"
DOC=cargo doc --no-deps --document-private-items
TEST=RUST_BACKTRACE=full cargo test --features="helpers"
BENCH=RUST_BACKTRACE=full cargo bench
FEATURE_SETS="crypto_k256" "crypto_k256,casper" "crypto_secp256k1" "crypto_secp256k1,casper" "crypto_secp256k1,casper-test" "crypto_secp256k1,radix" "solana" "radix" "soroban" "default-crypto"
WASM32_FEATURE_SETS="solana" "radix"

RUST_SDK_DIR=crates/redstone

prepare:
	@rustup target add wasm32-unknown-unknown
	@rustup component add clippy rustfmt
	cargo install wasm-bindgen-cli wasm-pack --locked

test: clippy
	@set -e; \
	for features in $(WASM32_FEATURE_SETS); do \
		echo "Running tests with features: $$features"; \
		(cd $(RUST_SDK_DIR) && wasm-pack test --node --no-default-features --features="helpers" --features=$$features); \
	done
	@set -e; \
	for features in $(FEATURE_SETS); do \
		echo "Running tests with features: $$features"; \
		($(TEST) --features=$$features); \
	done

bench:
	($(BENCH) --all-features);

docs:
	@set -e; \
	for features in $(FEATURE_SETS); do \
		echo "Documenting redstone with features: $$features"; \
		(rm -rf ./target/doc && $(DOC) --features=$$features && mkdir -p ./target/rust-docs/redstone && cp -r ./target/doc ./target/rust-docs/redstone/$$features); \
	done
	@set -e; \
	for features in $(WASM32_FEATURE_SETS); do \
		echo "Documenting redstone with features: $$features"; \
		(rm -rf ./target/doc && $(DOC) --features=$$features && mkdir -p ./target/rust-docs/redstone && cp -r ./target/doc ./target/rust-docs/redstone/$$features); \
	done

coverage:
	cargo install grcov --version=0.5.15
	CARGO_INCREMENTAL=0 \
		RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
		RUSTDOCFLAGS="-Cpanic=abort" cargo build --features="crypto_k256"
	CARGO_INCREMENTAL=0 \
		RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
		RUSTDOCFLAGS="-Cpanic=abort" $(TEST) --features="crypto_k256"

clippy: prepare
	@set -e; \
	for features in $(FEATURE_SETS); do \
		echo "Running clippy with features: $$features"; \
		($(CLIPPY) --all-targets --features=$$features -- -D warnings); \
	done
	@echo 'Check all features enabled'
	$(CLIPPY) --all-targets --all-features -- -D warnings
	@echo 'Check all features disabled'
	$(CLIPPY) --no-default-features --all-features -- -D warnings

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt
