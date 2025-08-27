#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn saludar(nombre: String) -> String {
    format!("Hola, {}! 👋 Desde Rust con Tauri 🚀", nombre)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![saludar])
        .run(tauri::generate_context!())
        .expect("error al ejecutar la app Tauri");
}
