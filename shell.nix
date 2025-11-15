{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy
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
