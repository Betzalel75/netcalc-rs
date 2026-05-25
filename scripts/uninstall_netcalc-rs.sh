#!/bin/bash

set -e

OS="$(uname -s)"
case "$OS" in
  Linux)  OS="linux" ;;
  Darwin) OS="macos" ;;
  *)      echo "[-] OS non supporté" ; exit 1 ;;
esac

APP_DIR="$HOME/.local/netcalc-rs.app"
BIN_LINK="$HOME/.local/bin/netcalc-rs"

echo "[!] Désinstallation de NetCalc-rs ($OS)"

rm -f "$BIN_LINK"
echo "[-] Lien supprimé : $BIN_LINK"

if [ -d "$APP_DIR" ]; then
  rm -rf "$APP_DIR"
  echo "[-] Dossier supprimé : $APP_DIR"
fi

if [ "$OS" = "linux" ]; then
  DESKTOP_FILE="$HOME/.local/share/applications/netcalc-rs.desktop"
  ICON_FILE="$HOME/.local/share/icons/netcalc-rs.png"
  rm -f "$DESKTOP_FILE" && echo "[-] Entrée de menu supprimée"
  rm -f "$ICON_FILE"    && echo "[-] Icône supprimée"
fi

echo "[✔] NetCalc-rs désinstallé."
