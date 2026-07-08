# Wallpaper Shuffler
This is a recursive solution to automating wallpaper shuffle while also supporting multiple directories by starting in `~/Pictures/Wallpapers`, randomly picking any of the available choices, then using walkdir to find the new wallpaper from there.

## About
The primary issue with how most operating systems, desktop environments, and window managers natively handle wallpaper shuffling is that they either require you to organize your wallpapers into a single lump sum, or force you to manually choose which specific directory it will pick from - which is tedious to say the least.

The purpose of this software is to randomly choose a genre category, then select an image from within that category while also keeping a rolling list of the seven most recent images in order to avoid seeing the same image within the span of a week.

---

## Supported Environments

| Operating System / Desktop Environment | Command / Protocol Used | Status |
| :--- | :--- | :--- |
| **Microsoft Windows** | Windows Desktop API |  Supported |
| **GNOME Desktop** | Gsetttings |  Supported |
| **KDE Plasma 6** | `qdbus6` |  Supported |
| **Niri** | DMS, Noctalia, Swaybg, awww  |  Supported |
| **Hyprland** | DMS, Noctalia, Swaybg, awww | Supported |
| **MangoWM** | DMS, Noctalia, Swaybg, awww | Supported |

---
## Configuration & History
Choose your operating system to see where configuration and history files are stored:

<details>
<summary><b>🪟 Windows Paths</b></summary>

* **Configuration File (Weights):** `%USERPROFILE%\.config\Wallpaper_Shuffler\config.toml`
* **History Log:** `%USERPROFILE%\.local\share\Wallpaper_Shuffler\history.toml`

</details>

<details>
<summary><b>🐧 Linux Paths (XDG Standard)</b></summary>

* **Configuration File (Weights):** `$XDG_CONFIG_HOME/Wallpaper_Shuffler/config.toml`
* **History Log:** `$XDG_DATA_HOME/Wallpaper_Shuffler/history.toml`

</details>

---

## Automation & Setup
Expand the section corresponding to your desktop environment to set up compilation and automatic cycling.

<details>
<summary><b>🪟 Windows Setup (Task Scheduler)</b></summary>

### Compilation
1. run `. install.ps1`.
2. install.ps1 will compile the source code and create a basic task on task scheduler.
3. by default, the task will be run daily at 00:00 or as soon as possible if scheduled time was missed.
</details>

<details>
<summary><b>🐧 Linux</b></summary>

### Compilation
1. in the project root, run `bash install.sh`
2. install.sh will handle the automation for you by compiling the binary, generating the systemd service and timer files and moving them to `~/.config/systemd/user/` then reloading the daemons via `systemctl --user daemon-reload` and enabling the timer.
3. by default, wallpapers will shuffle once a day. if you want it to shuffle more or less often you will need to edit the timer.

* **Note**: Using install.sh assumes that you use systemd. If you use a different system then it is up to you to figure out automation.
</details>

## Dependencies
### Requires:
* **All Platforms:** Rust
* **Windows:** Microsoft Visual Studio

## Note
This project assumes that you placed all of your images in `~/Pictures/Wallpapers/` or `~\Pictures\Wallpapers\` but if you did not then you will need to modify main.rs on line 26: and 27: to point to your location instead.


