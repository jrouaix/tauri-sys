#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{Manager, State};
use tauri_plugin_log::{LogTarget, LoggerBuilder};

struct Received(AtomicBool);
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn verify_receive(emitted: State<Received>) -> bool {
    emitted.0.load(Ordering::Relaxed)
}

#[tauri::command]
fn exit_with_error(e: &str) -> bool {
    eprintln!("{}", e);
    std::process::exit(1);
}

fn main() {
    let log_plugin = {
        let targets = [
            LogTarget::LogDir,
            #[cfg(debug_assertions)]
            LogTarget::Stdout,
            #[cfg(debug_assertions)]
            LogTarget::Webview,
        ];

        LoggerBuilder::new().targets(targets).build()
    };

    tauri::Builder::default()
        .plugin(log_plugin)
        .invoke_handler(tauri::generate_handler![verify_receive, exit_with_error])
        .setup(|app| {
            app.manage(Received(AtomicBool::new(false)));

            let app_handle = app.handle();
            app.listen_global("foo", move |_| {
                app_handle
                    .state::<Received>()
                    .0
                    .store(true, Ordering::Relaxed);
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
