#!/usr/bin/env bash

set -e

REPO="Betzalel75/netcalc-rs"
VERSION="${1:-latest}"
TMP_DIR="$HOME/.tmp/netcalc-rs-install"
INSTALL_DIR="$HOME/.local/netcalc-rs.app"
BIN_DIR="$HOME/.local/bin"

# Détection de l'OS et de l'architecture
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

# Mapper l'architecture vers le suffixe des releases
case "$ARCH" in
  x86_64|amd64)  ARCH_SUFFIX="x86_64" ;;
  aarch64|arm64) ARCH_SUFFIX="aarch64" ;;
  *)
    echo "[-] Architecture non supportée : $ARCH"
    exit 1
    ;;
esac

echo "[*] Système détecté : $OS / $ARCH"

# ── Dépendances système ──────────────────────────────────────────────
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

# ── Téléchargement ───────────────────────────────────────────────────
echo "[*] Téléchargement de NetCalc-rs ($VERSION)"
mkdir -p "$TMP_DIR"
cd "$TMP_DIR"

# Construction de l'URL de téléchargement
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

# ── Extraction ───────────────────────────────────────────────────────
echo "[-] Extraction..."
tar -Jxf archive.tar.xz
cd app/

# ── Installation ─────────────────────────────────────────────────────
echo "[-] Installation dans $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"
cp netcalc-rs* "$INSTALL_DIR/" 2>/dev/null || cp -r . "$INSTALL_DIR/"

mkdir -p "$BIN_DIR"
ln -sf "$INSTALL_DIR/netcalc-rs" "$BIN_DIR/netcalc-rs"
echo "[✔] Binaire installé : $BIN_DIR/netcalc-rs"

# ── Intégration au système ───────────────────────────────────────────
if [ "$OS" = "linux" ]; then
  DESKTOP_DIR="$HOME/.local/share/applications"
  ICON_DIR="$HOME/.local/share/icons"
  mkdir -p "$DESKTOP_DIR" "$ICON_DIR"
  if [ -f debian/netcalc-rs.desktop ]; then
    install -Dm 644 debian/netcalc-rs.desktop "$DESKTOP_DIR/netcalc-rs.desktop"
    echo "[✔] Entrée de menu ajoutée"
  fi
  if [ -f assets/images/netcalc-rs.png ]; then
    install -Dm 644 assets/images/netcalc-rs.png "$ICON_DIR/netcalc-rs.png"
    echo "[✔] Icône ajoutée"
  fi
elif [ "$OS" = "macos" ]; then
  # Sur macOS, optionnel : copier l'icône
  if [ -f assets/images/netcalc-rs.png ]; then
    mkdir -p "$INSTALL_DIR"
    cp assets/images/netcalc-rs.png "$INSTALL_DIR/icon.png"
  fi
  echo "[✔] NetCalc-rs prêt pour macOS"
  echo "    Astuce : créez un alias dans /Applications avec :"
  echo "    ln -sf $BIN_DIR/netcalc-rs /Applications/NetCalc-rs"
fi

# ── Nettoyage ────────────────────────────────────────────────────────
cd "$HOME"
rm -rf "$TMP_DIR"
echo "[✔] Installation terminée !"

# ── Instructions PATH ────────────────────────────────────────────────
echo ""
echo "[!] Assurez-vous que ~/.local/bin est dans votre PATH :"
case "$SHELL" in
  *zsh)  echo "    echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.zshrc && source ~/.zshrc" ;;
  *fish) echo "    fish_add_path -U \$HOME/.local/bin" ;;
  *)     echo "    echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc && source ~/.bashrc" ;;
esac
echo ""
echo "[✓] Lancez : netcalc-rs"
