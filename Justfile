# ==============================================================================

# Enter the Nix development shell. Ignore if already using direnv.
develop:
	nix develop

# Build the game.
build:
	cargo build

# Run the game.
run:
    cargo run -p have-a-sip

# ==============================================================================

# Format all the code.
[parallel]
fmt: fmt-justfile fmt-nix fmt-rust
    nixfmt flake.nix

[group('fmt')]
[private]
fmt-justfile:
    just --unstable --fmt

[group('fmt')]
[private]
fmt-nix:
    nixfmt flake.nix

[group('fmt')]
[private]
fmt-rust:
    cargo fmt

# ==============================================================================

# Lint all the code.
lint: lint-justfile lint-nix lint-rust

[group('lint')]
[private]
lint-justfile:
    just --unstable --fmt --check

[group('lint')]
[private]
lint-nix:
    nixfmt --check flake.nix

[group('lint')]
[private]
lint-rust:
    cargo fmt --check
    cargo clippy --workspace -- -D warnings
