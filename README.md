# Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```
### Dépendances:

*   `libgtk-3-dev`
*   `libsoup3-dev` (ou `libsoup-3.0-dev`, selon votre système)
*   `libjavascriptcoregtk-4.1-dev`
*   `libwebkit2gtk-4.1-dev`
*   `libxdo-dev`

Nom du repo : netcalc-rs
Description : 🦀 IPv4 Subnet Calculator | CIDR Converter | Network Address Utility 
               with CLI interface. Educational project for network engineers.
Topics : rust, networking, ip-calculator, subnetting, cidr