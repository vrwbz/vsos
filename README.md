# VSOS

A small x86_64 operating-system learning project written in Rust.

## First milestone

The kernel boots in QEMU and renders a lightweight VGA dashboard with kernel, architecture, runtime, display, and boot-service status panels. The command bar accepts `help`, `status`, `about`, `version`, and `clear`.

## Prerequisites

Install Rust through rustup, the nightly toolchain, QEMU, and Git. Then install the bootimage cargo subcommand:

```powershell
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
cargo +nightly install bootimage
```

## Build and run

```powershell
cargo +nightly bootimage
$env:Path += ";C:\Program Files\qemu"
cargo +nightly run
```

The final command launches QEMU through the runner configured in
[`.cargo/config.toml`](.cargo/config.toml). Close QEMU with `Ctrl+C` in the terminal.

## GitHub

Create an empty repository named `vsos` on GitHub, then run:

```powershell
git init
git add .
git commit -m "Create VSOS kernel"
git branch -M main
git remote add origin https://github.com/YOUR-USERNAME/vsos.git
git push -u origin main
```

## Roadmap

- Keyboard input and a command shell
- Interrupt handling and a timer
- Memory management
- A simple filesystem
- User programs
