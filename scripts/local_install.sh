#!/usr/bin/env bash
# local_install.sh: Install a locally built AppImage into ~/.local and
# register it in the application menu.
#
# Called by: make appimage-install
#
# What it does:
#   1. Copies the AppImage to ~/.local/<appname>/
#   2. Creates a symlink in ~/.local/bin/ so the app is on $PATH
#   3. Installs the .desktop file to ~/.local/share/applications/
#   4. Installs the icon to ~/.local/share/icons/
#   5. Refreshes the desktop database so the launcher appears immediately

set -euo pipefail

# ── helpers ────────────────────────────────────────────────────────────────

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

info() { printf "${GREEN}[+]${NC} %s\n" "$*"; }
die()  { printf "${RED}[x]${NC} %s\n" "$*" >&2; exit 1; }

# ── argument parsing ───────────────────────────────────────────────────────

if [[ $# -lt 3 ]]; then
    echo "Usage: $(basename "$0") <appimage> <desktop-file> <icon-file>"
    echo
    echo "Install a locally built AppImage into ~/.local/ for the current user."
    exit 1
fi

APPIMAGE_INPUT="$1"
DESKTOP_SRC="$2"
ICON_SRC="$3"

APPIMAGE_INPUT="$(realpath "$APPIMAGE_INPUT")"

[[ -f "$APPIMAGE_INPUT" ]] || die "AppImage not found: $APPIMAGE_INPUT"
[[ -x "$APPIMAGE_INPUT" ]] || die "AppImage is not executable: $APPIMAGE_INPUT"
[[ -f "$DESKTOP_SRC" ]]     || die ".desktop file not found: $DESKTOP_SRC"
[[ -f "$ICON_SRC" ]]        || die "icon file not found: $ICON_SRC"

# ── derive app name ────────────────────────────────────────────────────────

APPIMAGE_BASENAME="$(basename "$APPIMAGE_INPUT")"
APPNAME="$(echo "$APPIMAGE_BASENAME" | sed -E 's/\.AppImage$//; s/[-_][0-9].*$//')"

[[ -n "$APPNAME" ]] || die "could not derive app name from '$APPIMAGE_BASENAME'"

# ── directories ────────────────────────────────────────────────────────────

APPS_DIR="${HOME}/.local/${APPNAME}"
BIN_DIR="${HOME}/.local/bin"
APPS_DESKTOP="${HOME}/.local/share/applications"
APPS_ICONS="${HOME}/.local/share/icons"

# ── step 1: copy AppImage ─────────────────────────────────────────────────

mkdir -p "$APPS_DIR"
APPIMAGE_DEST="${APPS_DIR}/${APPIMAGE_BASENAME}"

if [[ "$APPIMAGE_INPUT" != "$APPIMAGE_DEST" ]]; then
    info "copying AppImage → $APPS_DIR/"
    cp "$APPIMAGE_INPUT" "$APPIMAGE_DEST"
    chmod +x "$APPIMAGE_DEST"
else
    info "AppImage already in $APPS_DIR/"
fi

# ── step 2: symlink into ~/.local/bin ─────────────────────────────────────

mkdir -p "$BIN_DIR"
SYMLINK="${BIN_DIR}/${APPNAME}"

rm -f "$SYMLINK"
info "symlink $SYMLINK → $APPIMAGE_DEST"
ln -s "$APPIMAGE_DEST" "$SYMLINK"

# ── step 3: install .desktop file ──────────────────────────────────────────

DESKTOP_DEST="${APPS_DESKTOP}/${APPNAME}.desktop"
mkdir -p "$APPS_DESKTOP"

info ".desktop → $DESKTOP_DEST"
install -Dm 644 "$DESKTOP_SRC" "$DESKTOP_DEST"

# ── step 4: install icon ───────────────────────────────────────────────────

ICON_EXT="${ICON_SRC##*.}"
ICON_DEST="${APPS_ICONS}/${APPNAME}.${ICON_EXT}"

info "icon → $ICON_DEST"
install -Dm 644 "$ICON_SRC" "$ICON_DEST"

sed -i "s|^Icon=.*|Icon=${ICON_DEST}|" "$DESKTOP_DEST"
sed -i "s|^Exec=.*|Exec=${SYMLINK}|" "$DESKTOP_DEST"

if command -v update-desktop-database >/dev/null 2>&1; then
    info "refreshing desktop database..."
    update-desktop-database "$APPS_DESKTOP" 2>/dev/null || true
fi

echo
info "done!"
echo "    binary   : $SYMLINK"
echo "    desktop  : $DESKTOP_DEST"
echo "    icon     : $ICON_DEST"
echo
echo "You may need to log out and back in for the launcher to appear."
