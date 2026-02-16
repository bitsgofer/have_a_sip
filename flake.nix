{
  description = "Game development env for Rust + Bevy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Platform detection
        isDarwin = pkgs.stdenv.isDarwin;
        isLinux = pkgs.stdenv.isLinux;

        # buildInputs: rustToolchain
        rustVersion = "1.93.0";
        rustTargets = [
          # Supported builds
          "x86_64-unknown-linux-gnu"
          "aarch64-unknown-linux-gnu"
          "x86_64-pc-windows-gnu"
          "aarch64-pc-windows-gnullvm"
          "x86_64-apple-darwin"
          "aarch64-apple-darwin"
          "wasm32-unknown-unknown"
        ];
        rustToolchain = pkgs.rust-bin.stable.${rustVersion}.default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
          ];
          targets = rustTargets;
        };

        # buildInputs: platform-specific tools
        commonTools = with pkgs; [
          git-lfs
          just
          shellcheck
          shfmt
        ];
        linuxTools = with pkgs; [
          pkg-config
          wayland
          alsa-lib
          alsa-plugins
          pipewire
          udev
          libxkbcommon
          vulkan-loader
          mesa
          libdrm
        ];
        darwinTools = with pkgs; [
          # TODO(exklamationmark): Add these when the time comes
          # darwin.apple_sdk.frameworks.Cocoa
          # darwin.apple_sdk.frameworks.CoreAudio
          # darwin.apple_sdk.frameworks.CoreFoundation
          # darwin.apple_sdk.frameworks.CoreGraphics
          # darwin.apple_sdk.frameworks.CoreVideo
          # darwin.apple_sdk.frameworks.Metal
          # darwin.apple_sdk.frameworks.QuartzCore
        ];

        # buildInputs: platform-specific shell hooks
        linuxShellHookBase = ''
          export LD_LIBRARY_PATH=${
            pkgs.lib.makeLibraryPath [
              pkgs.vulkan-loader
              pkgs.libxkbcommon
              pkgs.wayland
              pkgs.alsa-lib
              pkgs.pipewire
              pkgs.udev
              pkgs.libdrm
              pkgs.mesa
            ]
          }:$LD_LIBRARY_PATH

          export ALSA_PLUGIN_DIR=${pkgs.pipewire}/lib/alsa-lib
        '';
        linuxShellHookMesa = linuxShellHookBase + ''
          export VK_ICD_FILENAMES=${pkgs.mesa}/share/vulkan/icd.d/radeon_icd.x86_64.json
          export VK_LAYER_PATH=${pkgs.mesa}/share/vulkan/implicit_layer.d
        '';
        # NVIDIA: don't set VK_ICD_FILENAMES; the system NVIDIA driver
        # provides the Vulkan ICD at standard paths (e.g. /usr/share/vulkan/icd.d/).
        linuxShellHookNvidia = linuxShellHookBase;
        darwinShellHook = ""; # macOS uses Metal via wgpu => no Vulkan config

      in
      let
        commonShellHook = ''
          # All assets are put into: [repo root]/assets, so we configure Bevy
          # to always look there.
          export BEVY_ASSET_ROOT="$(git rev-parse --show-toplevel)"

          echo "Development env loaded!"
        '';
        commonBuildInputs = [
          rustToolchain
        ]
        ++ commonTools
        ++ pkgs.lib.optionals isLinux linuxTools
        ++ pkgs.lib.optionals isDarwin darwinTools;
      in
      {
        # Default shell (Mesa/AMD GPU)
        devShells.default = pkgs.mkShell {
          buildInputs = commonBuildInputs;
          shellHook = commonShellHook
            + (if isLinux then linuxShellHookMesa else darwinShellHook);
        };

        # NVIDIA GPU shell: nix develop .#nvidia
        devShells.nvidia = pkgs.mkShell {
          buildInputs = commonBuildInputs;
          shellHook = commonShellHook
            + (if isLinux then linuxShellHookNvidia else darwinShellHook);
        };
      }
    );
}
