#!/usr/bin/sh
set -eu

SERVICE_DIR="$HOME/.config/systemd/user"
BIN_DIR="$HOME/.local/bin"

# Check for NixOS
if [ -f /etc/NIXOS ] || { [ -f /etc/os-release ] && grep -qi "nixos" /etc/os-release; }; then
    IS_NIXOS=true
else
    IS_NIXOS=false
fi

# Dependencies check
if [ "$IS_NIXOS" = true ]; then
    if ! command -v nix-shell >/dev/null 2>&1; then
        printf '\033[1;91mNixOS detected, but "nix-shell" command was not found.\n\nPress ENTER to exit.\033[0m\n'
        read -r _
        exit 1
    fi
else
    if ! command -v rustc >/dev/null 2>&1; then
        printf '\033[1;91mMissing dependencies needed to compile.\n\nEnsure rust is installed.\nPress ENTER to exit.\033[0m\n'
        read -r _
        exit 1
    fi
fi

printf '\033[1;34m==> Building wall_shuff...\033[0m\n'

# Build using nix-shell
if [ "$IS_NIXOS" = true ] && [ -z "${IN_NIX_SHELL:-}" ]; then
    nix-shell --run "cargo build --release" || {
        printf '\033[1;91mBuild failed inside nix-shell.\033[0m\n'
        exit 1
    }
else
    cargo build --release || {
        printf '\033[1;91mBuild failed.\n\nCheck your Rust environment.\033[0m\n'
        exit 1
    }
fi

mkdir -p "$SERVICE_DIR" "$BIN_DIR"

printf '\033[1;34m==> Initializing automation...\033[0m\n'
mv "target/release/wall_shuff" "$BIN_DIR/"
if [ -f "target/release/wall_shuffd" ]; then
    mv "target/release/wall_shuffd" "$BIN_DIR/"
fi

mv "resources/wall_shuffd.service" "$SERVICE_DIR/"

systemctl --user daemon-reload
systemctl --user enable --now wall_shuffd.service

printf '\033[1;92mSuccess: Compilation & automation complete.\033[0m\n'
