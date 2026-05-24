#!/bin/bash

set -e

REPO="Betzalel75/netcalc-rs"
VERSION="${1:-latest}"
TMP_DIR="$HOME/.tmp/netcalc-rs-install"
INSTALL_DIR="$HOME/.local/netcalc-rs.app"
BIN_DIR="$HOME/.local/bin"
DESKTOP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons"
USER=$(whoami)

# Verifier les dependances
command -v curl >/dev/null 2>&1 || { echo >&2 "[-] curl est requis pour installer NetCalc-rs."; exit 1; }
command -v tar >/dev/null 2>&1 || { echo >&2 "[-] tar est requis pour installer NetCalc-rs."; exit 1; }

# Installer les dependances
echo "[*] Installation des dependances"
if dpkg -s "libxdo3" >/dev/null 2>&1; then
  echo "[✔] libxdo3 Ok"
else
  sudo apt-get install -y libxdo3
fi

echo "[*] Téléchargement de NetCalc-rs ($VERSION)"

# Création dossier temporaire
mkdir -p "$TMP_DIR"
cd "$TMP_DIR"

# Télécharger la dernière release
if [ "$VERSION" = "latest" ]; then
  RELEASE_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep "browser_download_url" | grep '\.tar\.xz' | cut -d '"' -f 4)
else
    RELEASE_URL="https://github.com/$REPO/releases/download/$VERSION/app.tar.xz"
fi

echo "[-] URL de téléchargement : $RELEASE_URL"
curl -LO "$RELEASE_URL"

# Extraire l'archive
echo "[-] Décompression xz..."
tar -Jxvf '*.tar.xz' 
cd app/


# Déplacer les fichiers
echo "[-] Déplacement des fichiers"
mkdir -p "$INSTALL_DIR"

mv netcalc-rs "$INSTALL_DIR/"

cp -r assets "$INSTALL_DIR/"

# Créer le lien symbolique
mkdir -p "$BIN_DIR"
ln -sf "$INSTALL_DIR/netcalc-rs" "$BIN_DIR/netcalc-rs"


mkdir -p "$DESKTOP_DIR" "$ICON_DIR"
install -Dm 644 debian/netcalc-rs.desktop "$DESKTOP_DIR/netcalc-rs.desktop"
install -Dm 644 assets/images/netcalc-rs.png "$ICON_DIR/netcalc-rs.png"
echo "[+] Application ajoutée au menu"


# Nettoyage
echo "[-] Nettoyage 🧹"
cd "$HOME"
sudo rm -rf "$TMP_DIR"

echo "[✔] Installation terminée ! Essayez : netcalc-rs"

echo
echo "[!] Pour exécuter 'netcalc-rs' depuis n'importe quel terminal, assurez-vous que '~/.local/bin' est dans votre PATH."
echo "    Voici comment faire :"

case "$SHELL" in
    *zsh)
        echo "    echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.zshrc"
        echo "    source ~/.zshrc"
        ;;
    *fish)
        echo "    fish_add_path -U \$HOME/.local/bin"
        ;;
    *)
        echo "    echo 'export PATH=\$HOME/.local/bin:\$PATH' >> ~/.bashrc"
        echo "    source ~/.bashrc"
        ;;
esac

echo
echo "[✓] Vous pouvez maintenant exécuter l'application avec : netcalc-rs"
