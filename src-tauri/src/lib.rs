// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn increase(count: i32) -> i32 {
    count + 1
}

#[tauri::command]
fn decrease(count: i32) -> i32 {
    count - 1
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![increase, decrease])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
