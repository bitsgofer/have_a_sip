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

# Run assets-related tasks
mod lfs 'justfiles/assets/assets.just'

# ==============================================================================

# Format all the code.
[parallel]
fmt: fmt-justfile fmt-nix fmt-rust fmt-scripts-bash
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

[group('fmt')]
[private]
fmt-scripts-bash:
    find . -name '*.bash' -exec shfmt -w {} +

# ==============================================================================

# Lint all the code.
lint: lint-justfile lint-nix lint-rust lint-scripts-bash

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
lint-scripts-bash:
    find . -name '*.bash' -exec shellcheck {} +

[group('lint')]
[private]
lint-rust:
    cargo fmt --check
    cargo clippy --workspace -- -D warnings
