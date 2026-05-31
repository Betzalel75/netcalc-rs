NAME=netcalc-rs
VERSION=$(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
UNAME_M=$(shell uname -m)
UNAME_S=$(shell uname -s | tr A-Z a-z)
export CARGO_BUILD_JOBS=4

# Compilation uniquement
build:
	dx bundle --release --platform desktop

# Compilation + archive tar.xz prête à déployer
release-local: build
	rm -rf app _build
	mkdir -p app
	cp target/dx/$(NAME)/bundle/$(UNAME_S)/appimage/$(NAME)_$(VERSION)_$(UNAME_M).AppImage app/
	cp -r assets app/
	mkdir -p _build
	archive="$(NAME)-$(VERSION)-$(UNAME_S)-$(UNAME_M).tar.xz"
	echo "[*] Création de $(archive)..."
	tar -cJf _build/$(archive) app/
	cd _build && sha256sum $(archive) > $(archive).sha256
	rm -rf app
	@echo "[✔] _build/$(archive)"
	@echo "[✔] _build/$(archive).sha256"

appimage-install: build
	@bash scripts/local_install.sh \
	target/dx/$(NAME)/bundle/$(UNAME_S)/appimage/$(NAME)_$(VERSION)_$(UNAME_M).AppImage \
	debian/$(NAME).desktop \
	assets/images/$(NAME).png

run-dev:
	cargo run

clean:
	rm -rf _build/ app/ *.tar.xz *.sha256

help:
	@echo "Cible           Description"
	@echo "------          -----------"
	@echo "build           Compilation release (cargo build --release)"
	@echo "release-local   Build + archive .tar.xz dans _build/"
	@echo "appimage-install Install l'AppImage localement sur le système"
	@echo "run-dev         Lancement en mode développement"
	@echo "clean           Nettoyage"
	@echo ""
	@echo "Publication multi-plateforme :"
	@echo "  git tag v1.1.0 && git push origin --tags"
	@echo "  → GitHub Actions construit toutes les cibles"

.PHONY: clean build release-local appimage-install run-dev help
