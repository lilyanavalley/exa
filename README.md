
# EXA


(Short for *Experiment A*.)
> ✏️ The codename '**EXA**' is subject to change once this project is nearing completion.

> 🚧 **This project is incomplete** and remains in an *unstable, unplayable state* until version number `1.0.0` is
> released. As such, you may *not* want to play the game in its current state, unless you'd like to test it or
> contribute your expertise to the development of this project. Additionally, be aware that your in-game progress may
> not be kept across different versions of this game.


## Developer's Guide

🤓 *This section uses verbiage for software/game developers and is not intended for the end-user.*

### Tooling

The majority of this project is written in [Rust](https://rust-lang.org) and uses the following tools as part of its
technical stack:

- [fyrox](https://crates.io/crates/fyrox); game engine
- [tracing](https://crates.io/crates/tracing); application tracing framework
- [tracy-client](https://crates.io/crates/tracy-client); application tracing with [Tracy](https://github.com/wolfpld/tracy) profiler
- [fluent-bundle](https://crates.io/crates/fluent-bundle); localization framework
- ...plus additional libraries listed in [CREDITS.md](./CREDITS.md) with accompanied explainations for their implementation.

----

**To prepare to compile this project**:

1. Install
[Rust and its package manager Cargo](https://www.rust-lang.org/tools/install).

2. 🪟 / 🍎 If you are compiling on **Windows** or **Mac OS**, no other dependencies are required.

3. 🐧 If you are compiling on a **GNU/Linux** system, please install
[Fyrox dependencies](https://fyrox-book.github.io/beginning/scripting.html#platform-specific-dependencies):

    ```bash
    sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev libxcb1-dev libxkbcommon-dev libasound2-dev build-essential
    ```
    (👆 Ubuntu/Debian-based system, using **apt**)

    (TODO: Add other package manager instructions.)

4. You are now ready to [Build](#buildrun) and [Install/Run](#installer) this project.


### Build/Run

Within this repository is a
[Cargo Workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html)
with sub-workspace packages, including:

- Executable application wrappers:
    - `executor` PC application
    - `executor-wasm` Web Assembly application
    - `executor-android` Android application (**disabled** [2])

- Engine components:
    - `game` Fyrox game plugin library[1]
    - `editor` Fyrox scene & UI editor

All of the `executor` packages reference the `game` library at compile-time and thus make **EXA** a multi-platform app.
The `editor` package is the [Fyrox Editor](https://fyrox-book.github.io/beginning/editor_overview.html).

#### Build Process

To build the PC executor (*without release optimizations*):

```bash
cargo build --package executor
```

To build the PC executor (*with release optimizations*):

```bash
cargo build --package executor --release
```

To build the editor:

```bash
cargo build --package editor --release
```

#### Run Process

To run the PC executor (*without optimizations*):

```bash
cargo run --package executor
```

To run the PC executor (*with optimizations*):

```bash
cargo run --package executor --release
```

To run the editor:

```bash
cargo run --package editor --release
```

### Installer

There is a ['packager' tool](https://crates.io/crates/cargo-packager)
available to create installable binaries for personal computers.


## Footnotes

[1]: Libraries cannot be executed directly.

[2]: Android is not currently supported by **EXA**.


## License

**EXA** is an *'Open Source' software project* and comes with an
[issues page](https://github.com/lilyanavalley/exa/issues), [contributor's guide](./CONTRIBUTE.md) and
[change log](./CHANGES.md) for you to peruse at your leisure.

Licensed under the *GNU General Public License, Version 3*.

![gpl3](./data/ui/gplv3-with-text-136x68.png "GPL-3.0 OSS License Logo")
