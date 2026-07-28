#!/bin/bash

if ! command -v rustc &>/dev/null; then
    echo -e "Missing dependencies needed to compile.\nEnsure rust is installed.\nPRESS ENTER to exit."
    read -r
    exit 1
fi

SERVICE_DIR="$HOME/.config/systemd/user"
APP_DIR="$HOME/.local/share/applications"
BIN_DIR="$HOME/.local/bin"

echo "Building wall_shuff..."
cargo build --release || {
    echo "Build failed. Check your Rust environment."
    exit 1
}

mkdir -p "$SERVICE_DIR" "$APP_DIR" "$BIN_DIR"

mv "target/release/wall_shuff" "$BIN_DIR/"
mv "resources/wall_shuffd.timer" "resources/wall_shuffd.service" "$SERVICE_DIR/"
mv "resources/wall_shuff.desktop" "$APP_DIR/"

systemctl --user daemon-reload
systemctl --user enable --now wall_shuffd.timer
