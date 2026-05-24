NAME=netcalc-rs
VERSION=$(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
BUILD=$(shell git rev-parse --short HEAD 2>/dev/null || echo "unknown")

# -- Targets -------------------------------------------------------------------

clean:
	rm -rf _build/ release/ app/ *.tar.xz

# Compilation release
build:
	cargo build --release

# Compilation + tar.xz portable (local)
build-local: build
	rm -rf app
	mkdir -p app/debian
	cp target/release/$(NAME) app/
	cp -r assets app/
	cp debian/netcalc-rs.desktop app/debian/
	tar -cJf $(NAME)-$(VERSION)-$(shell uname -m).tar.xz app/
	rm -rf app
	@echo "[✔] $(NAME)-$(VERSION)-$(shell uname -m).tar.xz créé"

# Build + archive dans _build/ avec checksum
build-portable: build
	mkdir -p _build
	rm -rf app
	mkdir -p app/debian
	cp target/release/$(NAME) app/
	cp -r assets app/
	cp debian/netcalc-rs.desktop app/debian/
	# Archive versionnée
	tar -cJf _build/$(NAME)-$(VERSION)-$(shell uname -m).tar.xz app/
	# Alias app.tar.xz pour compatibilité script d'install
	cp _build/$(NAME)-$(VERSION)-$(shell uname -m).tar.xz _build/app.tar.xz
	rm -rf app
	cd _build && sha256sum *.tar.xz > sha256sums.txt
	@echo "[✔] _build/$(NAME)-$(VERSION)-$(shell uname -m).tar.xz"
	@echo "[✔] _build/app.tar.xz (alias)"

# Cross-compilation via cross (Docker requis)
# cargo install cross
build-all:
	mkdir -p _build
	@archives=""; \
	for target in \
		x86_64-unknown-linux-gnu \
		aarch64-unknown-linux-gnu \
		x86_64-apple-darwin \
		x86_64-pc-windows-gnu; do \
		echo "[*] Cross-compilation pour $$target..."; \
		cross build --release --target $$target; \
		rm -rf _build/app; \
		mkdir -p _build/app/debian; \
		if [ -f target/$$target/release/$(NAME).exe ]; then \
			cp target/$$target/release/$(NAME).exe _build/app/; \
		else \
			cp target/$$target/release/$(NAME) _build/app/; \
		fi; \
		cp -r assets _build/app/; \
		cp debian/netcalc-rs.desktop _build/app/debian/; \
		archive="$(NAME)-$(VERSION)-$$target.tar.xz"; \
		cd _build && tar -cJf $$archive app/ && cd ..; \
		archives="$$archives $$archive"; \
		rm -rf _build/app; \
		echo "[✔] $$archive"; \
	done; \
	# Créer app.tar.xz depuis la première archive linux
	first_linux=$$(cd _build && ls *linux*.tar.xz 2>/dev/null | head -1); \
	if [ -n "$$first_linux" ]; then \
		cp "_build/$$first_linux" "_build/app.tar.xz"; \
		echo "[✔] _build/app.tar.xz (alias vers $$first_linux)"; \
	fi; \
	cd _build && sha256sum *.tar.xz > sha256sums.txt
	@echo "[✔] Tout les artéfacts sont dans _build/"

run-dev:
	cargo run

# Release GitHub — publique (pas de -d, pas de pre-release)
# Devient automatiquement la "latest" car tag > précédent
# Nécessite : gh CLI authentifié
release:
	mkdir -p release
	cp _build/*.tar.xz _build/sha256sums.txt release/
	cd release && sha256sum --quiet --check sha256sums.txt
	cd release && gh release create v$(VERSION) \
		-t "v$(VERSION)" \
		--notes "Release v$(VERSION)" \
		*.tar.xz sha256sums.txt
	@echo "[✔] Release v$(VERSION) publiée — c'est maintenant la latest"

help:
	@echo "Cible               Description"
	@echo "------              -----------"
	@echo "build               Compilation release simple"
	@echo "build-local         Build + tar.xz portable (dans le projet)"
	@echo "build-portable      Build + tar.xz dans _build/ + checksum"
	@echo "build-all           Cross-compilation multi-plateforme (cross)"
	@echo "run-dev             Lancement en mode développement"
	@echo "release             Publie une release GitHub (latest)"
	@echo "clean               Nettoyage"
	@echo ""
	@echo "Artefacts produits :"
	@echo "  netcalc-rs-<version>-<target>.tar.xz"
	@echo "  app.tar.xz  (alias, compatible script install)"

.PHONY: clean build build-local build-portable build-all run-dev release help
