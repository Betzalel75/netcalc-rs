
---

# 🧮 netcalc-rs

![License: GPL v3](https://img.shields.io/badge/license-GPLv3-blue)

**netcalc-rs** est une application de calcul d'adressage IP avec interface graphique, développée en Rust avec le framework Dioxus. Elle permet de comprendre et manipuler facilement les concepts de sous-réseaux, masques, adresses de diffusion, et plus encore.

> 🎯 Conçue pour un usage **desktop uniquement**, dans un style visuel **Glassmorphism** moderne, avec **support automatique du thème système clair/sombre**.

---

## ✨ Fonctionnalités

- Calcul automatique :
  - Adresse réseau
  - Adresse de broadcast
  - Masque de sous-réseau
  - Plage d'adresses
  - Subnetting dynamique
- Interface en **mode tableau de bord (Dashboard)**
- **Widgets explicatifs interactifs** avec visualisations (barres, bulles…)
- Modales de théorie intégrée (formules, rappels de concepts…)
- **Défilement unique par sections** (one-page scroll)
- Mode clair/sombre intégré avec détection automatique du système

---

## 🖼️ Aperçu

_(Ajoutez ici une capture d'écran ou une animation si disponible)_

---

## ⚙️ Installation

### Prérequis

- [Rust](https://www.rust-lang.org/fr)
- [Dioxus CLI](https://dioxuslabs.com/)
- Dépendances:
  - `libgtk-3-dev`
  - `libsoup3-dev` (ou `libsoup-3.0-dev`, selon votre système)
  - `libjavascriptcoregtk-4.1-dev`
  - `libwebkit2gtk-4.1-dev`
  - `libxdo-dev`

```bash
cargo install dioxus-cli
````

---

### Compilation

```bash
cargo build --release
```

L'exécutable sera généré dans `target/release/netcalc-rs`.

---

### Lancement en mode développement

```bash
dx serve
```

---

## 🖥️ Utilisation

L'application s'ouvre dans une fenêtre graphique. Entrez une adresse IP avec masque CIDR (ex: `192.168.1.0/24`), et naviguez dans les sections pour explorer les détails du sous-réseau, visualiser les adresses disponibles, etc.

---

## 📜 Licence

Ce projet est sous licence **GPL v3**. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

---
