#!/bin/bash

set -e

APP_DIR="$HOME/.local/netcalc-rs.app"
BIN_LINK="$HOME/.local/bin/netcalc-rs"
DESKTOP_FILE="$HOME/.local/share/applications/netcalc-rs.desktop"
ICON_FILE="$HOME/.local/share/icons/netcalc-rs.png"

echo "[!] Cette opération va désinstaller NetCalc-rs de votre système."

# Supprimer le lien symbolique
if [ -L "$BIN_LINK" ]; then
    echo "[-] Suppression du lien binaire $BIN_LINK"
    rm "$BIN_LINK"
fi

# Supprimer le dossier de l'application
if [ -d "$APP_DIR" ]; then
    echo "[-] Suppression du dossier de l'application $APP_DIR"
    rm -rf "$APP_DIR"
fi

# Supprimer le fichier .desktop
if [ -f "$DESKTOP_FILE" ]; then
    echo "[-] Suppression de l'entrée de menu $DESKTOP_FILE"
    rm "$DESKTOP_FILE"
fi

# Supprimer l’icône
if [ -f "$ICON_FILE" ]; then
    echo "[-] Suppression de l’icône $ICON_FILE"
    rm "$ICON_FILE"
fi

echo "[✔] NetCalc-rs a été désinstallé proprement.🧹"
