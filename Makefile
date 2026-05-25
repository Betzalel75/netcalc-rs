NAME=netcalc-rs
VERSION=$(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
UNAME_M=$(shell uname -m)
UNAME_S=$(shell uname -s | tr A-Z a-z)

# Compilation uniquement
build:
	cargo build --release

# Compilation + archive tar.xz prête à déployer
release-local: build
	rm -rf app _build
	mkdir -p app
	cp target/release/$(NAME) app/
	cp -r assets app/
	mkdir -p _build
	archive="$(NAME)-$(VERSION)-$(UNAME_S)-$(UNAME_M).tar.xz"
	echo "[*] Création de $(archive)..."
	tar -cJf _build/$(archive) app/
	cd _build && sha256sum $(archive) > $(archive).sha256
	rm -rf app
	@echo "[✔] _build/$(archive)"
	@echo "[✔] _build/$(archive).sha256"

run-dev:
	cargo run

clean:
	rm -rf _build/ app/ *.tar.xz *.sha256

help:
	@echo "Cible           Description"
	@echo "------          -----------"
	@echo "build           Compilation release (cargo build --release)"
	@echo "release-local   Build + archive .tar.xz dans _build/"
	@echo "run-dev         Lancement en mode développement"
	@echo "clean           Nettoyage"
	@echo ""
	@echo "Publication multi-plateforme :"
	@echo "  git tag v1.1.0 && git push origin --tags"
	@echo "  → GitHub Actions construit toutes les cibles"

.PHONY: clean build release-local run-dev help
