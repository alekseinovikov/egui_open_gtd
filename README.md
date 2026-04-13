# egui Open GTD

A [GTD](https://en.wikipedia.org/wiki/Getting_Things_Done) application built with [egui](https://github.com/emilk/egui) and [eframe](https://github.com/emilk/egui/tree/master/crates/eframe).

Supported platforms: **Desktop** (Linux, macOS, Windows) and **Android**.

---

## Prerequisites

- [Rust](https://rustup.rs/) toolchain (see `rust-toolchain` for the pinned version)

For Android builds:
- [Android SDK](https://developer.android.com/studio) with NDK installed
- Set `ANDROID_NDK_HOME` environment variable to your NDK path (e.g. `~/Android/Sdk/ndk/<version>`)
- [`cargo-apk`](https://github.com/rust-mobile/cargo-apk): `cargo install cargo-apk`

---

## Project Structure

This is a Cargo workspace:

- `android/` — shared app library (TemplateApp) + Android entry point (cdylib)
- `desktop/` — desktop binary entry point

## Desktop

### Run in development mode

```bash
cargo run -p eframe_template_desktop
```

### Build release binary

```bash
cargo build -p eframe_template_desktop --release
```

The binary is placed at `target/release/eframe_template_desktop`.

---

## Android

### Setup

1. Install Android SDK and NDK via Android Studio or the command-line tools.
2. Export the NDK path:
   ```bash
   export ANDROID_NDK_HOME=~/Android/Sdk/ndk/<version>
   export ANDROID_HOME=~/Android/Sdk
   ```
3. Install `cargo-apk`:
   ```bash
   cargo install cargo-apk
   ```
4. Add Android Rust targets (done automatically via `rust-toolchain`, or manually):
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
   ```

Android builds must be run from the `android/` subdirectory (cargo-apk does not support workspace root manifests).

### Debug build (for device testing)

```bash
cd android
cargo apk build
cargo apk run    # builds + installs on connected device/emulator
```

The debug APK is signed automatically with the Android debug keystore.

### Release build (requires a signing keystore)

Generate a keystore once:
```bash
keytool -genkey -v -keystore android/release.keystore -alias my_app \
  -keyalg RSA -keysize 2048 -validity 10000
```

Then configure `android/Cargo.toml` (`[package.metadata.android.signing.release]`) with the path and passwords, and build:

```bash
cd android
cargo apk build --release
```

The APK is placed at `target/release/apks/eframe_template.apk`.

### Run on a connected device or emulator

```bash
cd android
cargo apk run --release
```

This builds the APK and installs it on the first available ADB device/emulator.

### Build for a specific ABI

Edit `[package.metadata.android] build_targets` in `android/Cargo.toml`, or pass the target explicitly:

```bash
cd android
cargo apk build --release --target aarch64-linux-android   # ARM64 (most modern phones)
cargo apk build --release --target armv7-linux-androideabi  # ARM 32-bit (older phones)
cargo apk build --release --target x86_64-linux-android     # x86_64 (emulator)
```

---

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
