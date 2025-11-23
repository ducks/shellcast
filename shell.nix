{ pkgs ? import <nixpkgs> {} }:

let
  # Use latest stable Rust
  rust-overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs' = import <nixpkgs> { overlays = [ rust-overlay ]; };
  rust = pkgs'.rust-bin.stable.latest.default;
in

pkgs'.mkShell {
  buildInputs = with pkgs'; [
    # Rust toolchain (latest stable)
    rust
    rust-analyzer

    # Audio libraries
    pkg-config
    alsa-lib
    openssl

    # Build tools
    gcc
  ];

  shellHook = ''
    echo "shellcast development environment"
    echo "Rust version: $(rustc --version)"
  '';

  # Set environment variables for audio libraries
  ALSA_PLUGIN_DIR = "${pkgs.alsa-lib}/lib/alsa-lib";
  PKG_CONFIG_PATH = "${pkgs.alsa-lib.dev}/lib/pkgconfig:${pkgs.openssl.dev}/lib/pkgconfig";
}
