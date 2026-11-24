# KAKURU

KAKURU is a native Rust application for Windows and Android that helps with
common mathematics calculations. Its UI is built with egui/eframe; calculations
run locally on the device.

## Requirements

- Rust stable toolchain
- Windows: MinGW-w64 GCC toolchain
- Android: Android SDK, NDK, and `cargo-apk`

Install `cargo-apk` with:

```shell
cargo install cargo-apk
```

## Run and test on Windows

```shell
cargo run
cargo test
```

## Build

```shell
cargo build --release
cargo apk build --lib
```

The Android package metadata and supported ARM targets are configured in
`Cargo.toml`. The feedback form opens the device's email app. Existing
calculation formulas and the SPLDV development placeholder are retained.
The APK command builds a development-signed package; configure a release
keystore in `Cargo.toml` before publishing an optimized APK to an app store.
