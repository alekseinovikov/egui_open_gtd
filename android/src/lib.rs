#![warn(clippy::all, rust_2018_idioms)]

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    eframe::run_native(
        "egui Open GTD",
        options,
        Box::new(|cc| Ok(Box::new(open_gtd_core::TemplateApp::new(cc)))),
    )
    .expect("Failed to run app");
}
