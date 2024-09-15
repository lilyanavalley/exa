
# Change Log


## v0.1
Introduces Fyrox game engine, tracing capabilities, editor & executor packages.

### 0.1.0 (Init)
This is the inception history of the project. Pretty bare-bones around here...

- Initialized with `fyrox-template` as a `3D` project.
- Added [changefile](./CHANGES.md).
- Added [.gitignore](./.gitignore).
- Added [tracy](./game/src/tracy.rs) for game tracing capabilities.

- Introduces dependencies:
    - [strum](https://crates.io/crates/strum) v0.26.3
    - [strum_macros](https://crates.io/crates/strum_macros) v0.26.3

- Start of CI/CD on [GitHub repo](https://github.com/lilyanavalley/exa) with files:
    - [.github/workflows/test.yaml](./.github/workflows/test.yaml) for testing game on push or P.R.
    - [.github/workflows/scheduled-build.yaml](./.github/workflows/scheduled-build.yaml) for scheduled[^1] builds.


[^1]: Builds every Monday, 23:00 EST (11:00 PM).
