// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        //.plugin(tauri_plugin_barcode_scanner::init())
        .setup(|app| {
            #[cfg(any(target_os = "android", target_os = "ios"))]
            app.handle().plugin(tauri_plugin_barcode_scanner::init())?;

            // #[cfg(target_os = "android")]
            // {
            //     use tauri::Manager;
            //     if let Some(window) = app.get_webview_window("main") {
            //     }
            // }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running application");
}
