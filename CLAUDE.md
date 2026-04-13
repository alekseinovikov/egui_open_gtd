# CLAUDE.md — Project Context

## Project

**egui Open GTD** — a GTD (Getting Things Done) productivity app built with [egui](https://github.com/emilk/egui) and [eframe](https://github.com/emilk/egui/tree/master/crates/eframe).

## Platform Targets

- **Desktop**: Linux, macOS, Windows
- **Android**: ARM64, ARMv7, x86_64 (via `android_main` + `cargo-apk`)
- **Web/WASM**: intentionally removed — do not re-add web/WASM support

## Workspace Structure

```
egui_open_gtd/
├── Cargo.toml           — workspace manifest (lints, profiles, patch)
├── android/             — shared library: TemplateApp logic + Android entry point
│   ├── Cargo.toml       — cdylib+rlib, android metadata (cargo-apk)
│   └── src/
│       ├── lib.rs       — pub use TemplateApp + android_main
│       └── app.rs       — TemplateApp UI implementation
├── desktop/             — desktop binary
│   ├── Cargo.toml       — depends on android lib via path
│   └── src/main.rs      — fn main() for Linux/macOS/Windows
└── assets/
    └── favicon-512x512.png  — desktop window icon
```

## Key Commands

```bash
# Desktop
cargo run -p eframe_template_desktop
cargo build -p eframe_template_desktop --release

# Android (run from android/ subdirectory — cargo-apk limitation)
cd android
cargo apk build            # debug APK
cargo apk build --release  # release APK (requires keystore config)
cargo apk run              # build + install on device/emulator
```

## Android Entry Point

`android/src/lib.rs` contains:
```rust
#[cfg(target_os = "android")]
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) { ... }
```

Android builds require `cargo-apk` installed and `ANDROID_HOME` + `ANDROID_NDK_ROOT` env vars set.
APK metadata (package name, SDK versions) is in `[package.metadata.android]` in `android/Cargo.toml`.

**Important:** `cargo apk` must be run from `android/` — it does not support workspace root manifests.

## No WASM

There is no `Trunk.toml`, `index.html`, or WASM build target. Do not suggest or add WASM/web dependencies.
