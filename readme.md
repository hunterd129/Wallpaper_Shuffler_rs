# Wallpaper Shuffler
This is a recursive solution to automating wallpaper shuffle while also supporting multiple directories by starting in `~\Pictures\Wallpapers`, randomly picking any of the available choices, then using walkdir to find the new wallpaper from there.

## About
The primary issue with how Windows natively handles wallpaper shuffling is that it either requires you to organize your wallpapers into a lump sum, or to manually choose which directory it will pick from which is tedious to say the least.

The purpose of this software is to randomly choose a genre category, then select an image from within that category while also keeping a rolling list of the seven most recent images in order to avoid seeing the same image within the span of a week.

## Configuration & History
Choose your operating system to see where configuration and history files are stored:

<details>
<summary><b>🪟 Windows Paths</b></summary>

* **Configuration File (Weights):** `%\USERPROFILE%\.config\Wallpaper_Shuffler\config.toml`
* **History Log:** `%\USERPROFILE%\.local\share\Wallpaper_Shuffler\history.toml`

</details>

<details>
<summary><b>🐧 GNOME / Linux Paths (XDG Standard)</b></summary>

* **Configuration File (Weights):** `~/.config/Wallpaper_Shuffler/config.toml`
* **History Log:** `~/.local/share/Wallpaper_Shuffler/history.toml`

</details>

---

## Automation & Setup
Expand the section corresponding to your desktop environment to set up compilation and automatic cycling.

<details>
<summary><b>🪟 Windows Setup (Task Scheduler)</b></summary>

### Compilation
1. Open PowerShell or Command Prompt in the project directory.
2. Compile the release binary:
   ```bash
   cargo build --release
3. Open Task Scheduler and create a basic task
4. Set the action to start a program and point it to the binary in `Wallpaper_Shuffler\target\release\Wallpaper_Shuffler.exe`
5. Set the interval the your preference. (e.g. hourly, daily, etc)
6. set it to run as soon as possible if scheduled time was missed in case your PC was sleeping at the time.

</details>

<details>
<summary><b>🐧 GNOME / Linux</b></summary>

### Compilation
1. in the project root, run `bash install.sh`
2. install.sh will handle the automation for you by compiling the binary, generating the systemd service and timer files and moving them to `~/.config/systemd/user/` then reloading the daemons via `systemctl --user daemon reload` and enabling the timer.

</details>

## Dependencies
### Requires:
* **All Platforms:** Rust
* **Windows:** Microsoft Visual Studio
* **GNOME / Linux:** Development headers for GLib/GIO (`glib-2.0`). Depending on your Linux distribution, install the corresponding package:

| Linux Distribution | Package Name | Installation Command |
| :--- | :--- | :--- |
| **Fedora-based** | `glib2-devel` | `sudo dnf install glib2-devel` |
| **Arch-based** | Built into `glib2` | *Already included with base system* |
| **Ubuntu/Debian-based** | `libglib2.0-dev` | `sudo apt install libglib2.0-dev` |
| **openSUSE** | `glib2-devel` | `sudo zypper install glib2-devel` |
| **Alpine Linux** | `glib-dev` | `apk add glib-dev` |

## Note
This project assumes that you placed all of your images in `~/Pictures/Wallpapers/`/`~\Pictures\Wallpapers\` but if you did not then you will need to modify main.rs on line 26: for pictures_dir to point to your location instead.


