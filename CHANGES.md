
# Change Log


## v0.1

Introduces Fyrox game engine, tracing capabilities, editor & executor packages.

### 0.1.0 (Init)

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

**Dialog Component**
Creation of interactive *Dialog* menu system. Intention to write a resuable interface for speaking to NPCs. Additional
features surrounding the dialog menu, including rich/markdown text may be implemented here or in a near-future version.

### 0.2.0

todo
