#!/bin/bash
#check for dependencies
if ! pkg-config --exists glib-2.0; then
    echo -e "Missing dependencies needed to compile.\n PRESS ENTER to exit."
    read -r
    exit 1
fi

if ! command -v rustc &>/dev/null; then
    echo -e "missing dependencies needed to compile.\n ensure rust is installed.\n PRESS ENTER to exit."
    read -r
    exit 1
fi

SERVICE_DIR="$HOME/.config/systemd/user"
APP_DIR="$HOME/.local/share/applications"
BIN_DIR="$HOME/.local/bin"

echo "Building Wallpaper_Shuffler..."
cargo build --release || {
    echo "Build failed. Check your Rust environment."
    exit 1
}
mkdir -p "$SERVICE_DIR" "$APP_DIR" "$BIN_DIR"

mv "target/release/Wallpaper_Shuffler" "$BIN_DIR"

# Generate wall_shuffd.service & .timer
cat <<EOF >"$SERVICE_DIR/wall_shuffd.service"
[Unit]
Description=Trigger Wallpaper swap

[Service]
Type=oneshot
ExecStart=%h/.local/bin/Wallpaper_Shuffler
StandardOutput=journal
StandardError=journal

SyslogIdentifier=wall_shuffd

[Install]
WantedBy=default.target
EOF

# 2. Generate the .desktop file
cat <<EOF >"$APP_DIR/Wallpaper_Shuffler.desktop"
[Desktop Entry]
Name=Wallpaper_Shuffler
Exec=$BIN_DIR/Wallpaper_Shuffler
Icon=media-playlist-shuffle
Type=Application
Terminal=false
Categories=Utility
EOF

cat <<EOF >"$SERVICE_DIR/wall_shuffd.timer"
[Unit]
Description=Schedule for wallpaper swap

[Timer]
OnCalendar=daily
Persistent=true
Unit=wall_shuffd.service

[Install]
WantedBy=timers.target
EOF

# Generate .desktop entry
cat <<EOF >"$APP_DIR/Wallpaper_Shuffler.desktop"
[Desktop Entry]
Name=Wallpaper_Shuffler
Exec=$BIN_DIR/Wallpaper_Shuffler
Icon=media-playlist-shuffle
Type=Application
Terminal=false
Categories=Utility
EOF

# Reload and enable
systemctl --user daemon-reload
systemctl --user enable --now Wallpaper_Shuffler.timer
