#!/usr/bin/env bash

set -e

REPO="Betzalel75/netcalc-rs"
VERSION="${1:-latest}"
TMP_DIR="$HOME/.tmp/netcalc-rs-install"
INSTALL_DIR="$HOME/.local/netcalc-rs.app"
BIN_DIR="$HOME/.local/bin"

OS="$(uname -s)"
ARCH="$(uname -m)"
case "$OS" in
  Linux)   OS="linux" ;;
  Darwin)  OS="macos"  ;;
  *)
    echo "[-] OS non supporté : $OS"
    echo "    Linux et macOS sont supportés."
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)  ARCH_SUFFIX="x86_64" ;;
  aarch64|arm64) ARCH_SUFFIX="aarch64" ;;
  *)
    echo "[-] Architecture non supportée : $ARCH"
    exit 1
    ;;
esac

echo "[*] Système détecté : $OS / $ARCH"

echo "[*] Vérification des dépendances"
command -v curl >/dev/null 2>&1 || { echo "[-] curl est requis."; exit 1; }
command -v tar  >/dev/null 2>&1 || { echo "[-] tar est requis."; exit 1; }

if [ "$OS" = "linux" ]; then
  if dpkg -s "libxdo3" >/dev/null 2>&1; then
    echo "[✔] libxdo3 déjà installé"
  else
    echo "[*] Installation de libxdo3..."
    sudo apt-get install -y libxdo3
  fi
fi

echo "[*] Téléchargement de NetCalc-rs ($VERSION)"
mkdir -p "$TMP_DIR"
cd "$TMP_DIR"

if [ "$OS" = "macos" ]; then
  TARGET="$ARCH_SUFFIX-apple-darwin"
else
  TARGET="$ARCH_SUFFIX-unknown-linux-gnu"
fi

if [ "$VERSION" = "latest" ]; then
  RELEASE_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
    | grep "browser_download_url" \
    | grep "$TARGET\.tar\.xz" \
    | head -1 \
    | cut -d '"' -f 4)
else
  RELEASE_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/tags/$VERSION" \
    | grep "browser_download_url" \
    | grep "$TARGET\.tar\.xz" \
    | head -1 \
    | cut -d '"' -f 4)
fi

if [ -z "$RELEASE_URL" ]; then
  echo "[-] Aucun téléchargement trouvé pour $OS / $VERSION"
  exit 1
fi

echo "[-] Téléchargement : $RELEASE_URL"
curl -sL -o archive.tar.xz "$RELEASE_URL"

echo "[-] Extraction..."
tar -Jxf archive.tar.xz
cd app/

mkdir -p "$BIN_DIR"

if [ "$OS" = "linux" ]; then
  echo "[-] Installation sous Linux..."
  mkdir -p "$INSTALL_DIR"

  APPIMAGE=$(find . -maxdepth 1 -name "*.AppImage" | head -1)

  if [ -n "$APPIMAGE" ]; then
    echo "[-] AppImage détecté ($ARCH_SUFFIX)"
    cp "$APPIMAGE" "$INSTALL_DIR/netcalc-rs.AppImage"
    chmod +x "$INSTALL_DIR/netcalc-rs.AppImage"
    ln -sf "$INSTALL_DIR/netcalc-rs.AppImage" "$BIN_DIR/netcalc-rs"
  else
    echo "[-] Binaire natif détecté ($ARCH_SUFFIX)"
    cp -r ./* "$INSTALL_DIR/" 2>/dev/null || true
    chmod +x "$INSTALL_DIR/netcalc-rs"
    ln -sf "$INSTALL_DIR/netcalc-rs" "$BIN_DIR/netcalc-rs"
  fi
  echo "[✔] Binaire lié dans : $BIN_DIR/netcalc-rs"

elif [ "$OS" = "macos" ]; then
  echo "[-] Installation sous macOS..."
  MAC_APP_DIR="$HOME/Applications"
  mkdir -p "$MAC_APP_DIR"

  APP_BUNDLE=$(find . -maxdepth 1 -type d -name "*.app" | head -1)

  if [ -n "$APP_BUNDLE" ]; then
    BUNDLE_NAME=$(basename "$APP_BUNDLE")
    echo "[-] Bundle macOS détecté : $BUNDLE_NAME"

    rm -rf "${MAC_APP_DIR:?}/${BUNDLE_NAME:?}"
    cp -R "$APP_BUNDLE" "$MAC_APP_DIR/"

    ln -sf "$MAC_APP_DIR/$BUNDLE_NAME/Contents/MacOS/netcalc-rs" "$BIN_DIR/netcalc-rs"
    echo "[✔] Application installée dans : $MAC_APP_DIR/$BUNDLE_NAME"
  else
    echo "[-] Erreur: Bundle .app introuvable pour macOS."
    exit 1
  fi
fi

if [ "$OS" = "linux" ]; then
  DESKTOP_DIR="$HOME/.local/share/applications"
  ICON_DIR="$HOME/.local/share/icons"
  mkdir -p "$DESKTOP_DIR" "$ICON_DIR"

  if [ -f debian/netcalc-rs.desktop ]; then
    sed -i "s|^Exec=.*|Exec=$BIN_DIR/netcalc-rs|" debian/netcalc-rs.desktop
    install -Dm 644 debian/netcalc-rs.desktop "$DESKTOP_DIR/netcalc-rs.desktop"
    echo "[✔] Entrée de menu ajoutée"
  fi

  if [ -f assets/icon.png ]; then
    install -Dm 644 assets/icon.png "$ICON_DIR/netcalc-rs.png"
    echo "[✔] Icône ajoutée"
  fi
fi

cd "$HOME"
rm -rf "$TMP_DIR"
echo "[✔] Installation terminée !"

echo ""
echo "[!] Assurez-vous que ~/.local/bin est dans votre PATH :"
case "$SHELL" in
  *zsh)  echo "    echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.zshrc && source ~/.zshrc" ;;
  *fish) echo "    fish_add_path -U \$HOME/.local/bin" ;;
  *)     echo "    echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc && source ~/.bashrc" ;;
esac
echo ""
echo "[✓] Lancez : netcalc-rs"
