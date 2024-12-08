
# Change Log


## v0.1

Introduces Fyrox game engine, tracing capabilities, editor & executor packages.

### v0.1.0 (Init)

This is the inception history of the project. Pretty bare-bones around here...

- Initialized with `fyrox-template` as a `3D` project.
- Added [changefile](./CHANGES.md).
- Added [.gitignore](./.gitignore).
- Added [tracy](./game/src/tracy.rs) for game tracing capabilities.

- Introduces the following `game/` objects:
    - [Player (script)](./game/src/player/mod.rs),
    - [Player Skybox](./game/src/player/skybox.rs) and,
    - [Settings](./game/src/settings.rs). 

- Introduces dependencies:
    - [strum](https://crates.io/crates/strum) v0.26.3
    - [strum_macros](https://crates.io/crates/strum_macros) v0.26.3
    - [gilrs](https://crates.io/crates/gilrs) v0.11.0
    - [serde](https://crates.io/crates/serde) v1.0.210
    - [ron](https://crates.io/crates/ron) v0.8.1
    - [fyrox](https://crates.io/crates/fyrox) v0.34.0
    - [firedbg-lib](https://crates.io/crates/firedbg-lib) v0.1.2
    - [tracing](https://crates.io/crates/tracing) v0.1.40
    - [tracy-client](https://crates.io/crates/tracy-client) v0.17
    - [tracy-client-sys](https://crates.io/crates/tracy-client-sys) v0.22
    - [fluent-bundle](https://crates.io/crates/fluent-bundle) v0.15.3

- Start of CI/CD on [GitHub repo](https://github.com/lilyanavalley/exa) with files:
    - [.github/workflows/test.yaml](./.github/workflows/test.yaml) for testing game on push or P.R.
    - [.github/workflows/scheduled-build.yaml](./.github/workflows/scheduled-build.yaml) for scheduled[^1] builds.

- Added GPL-3 license:
    - [OSS license file](./COPYING.md)
    - [GPL-3 logo](./data/ui/gplv3-with-text-136x68.png)

[^1]: Builds every Monday, 23:00 EST (11:00 PM).


## v0.2

Creation of interactive *Dialog* menu system. Intention to write a resuable interface for speaking to NPCs. Additional
features surrounding the dialog menu, including rich/markdown text may be implemented here or in a near-future version.

### v0.2.0 (dialog)
todo


## v0.3

Implementation of packaging tools for authoring installable game packages (like `.msi`, `.appimage`, `.dmg`...) This should hopefully make it easier to install a copy of **EXA** on the player's device and help resolve resources during runtime.

### v0.3.0 (packager)

- Introduces dependencies in `executor`:
    - [cargo-packager](https://crates.io/crates/cargo-packager) v0.11.2
    - [cargo-packager-resource-resolver](https://crates.io/crates/cargo-packager-resource-resolver) v0.1.2

- Implements functionality in `game`:
    - Receive a filesystem path from the running `executor`, pointing to the game's data directory.
    - Use a fallback technique to resolve the game's data directory if the game was not packaged using `cargo-packager`.

- Implements functionality in `executor`:
    - Sends a filesystem path to `game` if the game has been packaged and installed.

- Added assets:
    - [exa-32x.png](./data/ui/exa-32x.png) placeholder game icon.
    - [exa-64x.png](./data/ui/exa-64x.png) placeholder game icon.
    - [exa-128x.png](./data/ui/exa-128x.png) placeholder game icon.
    - [exa-256x.png](./data/ui/exa-256x.png) placeholder game icon.
    - [exa.pxo](./data/ui/exa.pxo) placeholder game icon source file.
