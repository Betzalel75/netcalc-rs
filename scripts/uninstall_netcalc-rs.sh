#!/bin/bash

set -e

OS="$(uname -s)"
case "$OS" in
  Linux)  OS="linux" ;;
  Darwin) OS="macos" ;;
  *)      echo "[-] OS non supporté" ; exit 1 ;;
esac

BIN_LINK="$HOME/.local/bin/netcalc-rs"
LINUX_APP_DIR="$HOME/.local/netcalc-rs.app"
MAC_APP_DIR="$HOME/Applications/NetcalcRs.app"

echo "[!] Désinstallation de NetCalc-rs ($OS)"

if [ -L "$BIN_LINK" ] || [ -f "$BIN_LINK" ]; then
  rm -f "$BIN_LINK"
  echo "[-] Lien supprimé : $BIN_LINK"
fi

if [ -d "$LINUX_APP_DIR" ]; then
  rm -rf "$LINUX_APP_DIR"
  echo "[-] Dossier supprimé : $LINUX_APP_DIR"
fi

if [ -d "$MAC_APP_DIR" ]; then
  rm -rf "$MAC_APP_DIR"
  echo "[-] Application macOS supprimée : $MAC_APP_DIR"
fi

if [ "$OS" = "linux" ]; then
  DESKTOP_FILE="$HOME/.local/share/applications/netcalc-rs.desktop"
  ICON_FILE="$HOME/.local/share/icons/netcalc-rs.png"
  
  if [ -f "$DESKTOP_FILE" ]; then
    rm -f "$DESKTOP_FILE"
    echo "[-] Entrée de menu supprimée"
  fi
  
  if [ -f "$ICON_FILE" ]; then
    rm -f "$ICON_FILE"
    echo "[-] Icône supprimée"
  fi
fi

echo "[✔] NetCalc-rs désinstallé."