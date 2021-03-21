# Rust playground

```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
rustup update

rustc --version

## Build project

```console
cargo new [PROJECT-NAME]
```

## Install Cargo dependencies

```console
cargo fetch
```
NB. Similar to npm install

### Compile main.rs
```console
cargo build --dev
```
--

### Compile and run main.rs
```console
    cargo run --release
```