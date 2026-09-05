#!/bin/bash

SERVICE_DIR="$HOME/.config/systemd/user"
BIN_DIR="$HOME/.local/bin"

# check if using NixOS
if [ -f /etc/NIXOS ] || ([ -f /etc/os-release ] && grep -qi "nixos" /etc/os-release); then
    IS_NIXOS=true
else
    IS_NIXOS=false
fi

# Dep check
if [ "$IS_NIXOS" = true ]; then
    if ! command -v nix-shell &>/dev/null; then
        echo -e "NixOS detected, but 'nix-shell' command was not found.\nPRESS ENTER to exit."
        read -r
        exit 1
    fi
else
    if ! command -v rustc &>/dev/null; then
        echo -e "Missing dependencies needed to compile.\nEnsure rust is installed.\nPRESS ENTER to exit."
        read -r
        exit 1
    fi
fi

echo "Building wall_shuff..."

# Build using nix-shell
if [ "$IS_NIXOS" = true ] && [ -z "$IN_NIX_SHELL" ]; then
    nix-shell --run "cargo build --release" || {
        echo "Build failed inside nix-shell."
        exit 1
    }
else
    cargo build --release || {
        echo "Build failed. Check your Rust environment."
        exit 1
    }
fi

mkdir -p "$SERVICE_DIR" "$BIN_DIR"

mv "target/release/wall_shuff" "$BIN_DIR/"
if [ -f "target/release/wall_shuffd" ]; then
    mv "target/release/wall_shuffd" "$BIN_DIR/"
fi

mv "resources/wall_shuffd.service" "$SERVICE_DIR/"

systemctl --user daemon-reload
systemctl --user enable --now wall_shuffd.service
