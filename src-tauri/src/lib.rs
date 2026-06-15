mod commands;
mod html_renderer;

use commands::*;
use tauri_plugin_updater::UpdaterExt;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let updater = match handle.updater() {
                    Ok(u) => u,
                    Err(e) => {
                        eprintln!("Updater init error: {}", e);
                        return;
                    }
                };
                match updater.check().await {
                    Ok(Some(update)) => {
                        if let Err(e) = update.download_and_install(
                            |_current, _total| {},
                            || {},
                        ).await {
                            eprintln!("Update install error: {}", e);
                        }
                    }
                    Ok(None) => {}
                    Err(e) => eprintln!("Update check error: {}", e),
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            pick_markdown_file,
            pick_markdown_save_path,
            read_file,
            write_file,
            show_error,
            pick_image,
            compile_pdf,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
