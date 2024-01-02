# Rust Todos

## Necessary packages

> on wsl2 ubuntu 22.04

```bash
sudo apt-get update -y
sudo apt-get install -y libx11-xcb-dev libxi-dev libxrandr-dev libxcursor-dev libx11-dev
```

## Run

```bash
cargo run
```

## Build for Windows

```bash
$ rustup target add x86_64-pc-windows-gnu
$ sudo apt-get install mingw-w64
$ cargo build --target x86_64-pc-windows-gnu
```