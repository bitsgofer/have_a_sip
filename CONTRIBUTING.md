# An open-source game

We are going to try something quite radical:
After a certain point, the source code for `Have a sip` will become public.
This will enable others to extend the game in the long run.

As such, we will attempt to treat the game's development like an open-source
project now.

> NOTE: We still charge a "convenience" fee if people download binaries
> from Steam and other 3rd-party stores.
> However, there should be no barrier if people want to build those themselves.

# Quick start

**Setup `nix`**

- [Install nix](https://nix.dev/install-nix).
- (Optional) [Install direnv](https://github.com/nix-community/nix-direnv?tab=readme-ov-file#installation)
  and [hook `direnv` into your shell](https://direnv.net/docs/hook.html).
- Inside the repo, run `nix develop`. Or have `direnv` automatically set it up (and refresh with `direnv reload`).
- That's all.

```
$> cd have_a_sip
$> nix develop    # Or: just develop (if 'just' is installed).
$> just run
```

# FAQs

- [Why do we use `nix`](docs/adr/001-reproducible-tools)?
- [Why do we use `just`](docs/adr/002-justfile-instead-of-makefile)?

# Repo structure

```
.
├── docs
│   └── adr         # Architecture Decision Record
├── flake.nix       # Packaged development tools
├── Justfile        # All tasks
├── Cargo.toml      # Shared depedencies of the project
├── game            # Game binary (Bevy app)
│   ├── Cargo.toml
│   └── src
└── simulation      # Game logic
    ├── Cargo.toml
    └── src
```
