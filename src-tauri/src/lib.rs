// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_pty::init()) // ajout du plugin pty permettant de demander à l'os de créer un pseudo terminal
        .plugin(tauri_plugin_dialog::init()) // ajout du plugin dialog permettant d'ouvrir l'explorateur de fichiers de l'os
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



