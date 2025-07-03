#![warn(clippy::pedantic, clippy::unwrap_used, clippy::nursery)]

use dotenv_codegen::dotenv;
use tauri_plugin_http::reqwest;

#[tauri::command]
fn greet(name: &str) -> String {
    if name.to_lowercase() == "doe" {
        String::from("Such a good doe for Miss~")
    } else {
        format!("Hello, {}! You've been greeted from Rust!", name)
    }
}

#[tauri::command]
async fn greet2(name: &str) -> Result<String, ()> {
    let result = reqwest::get(format!("{}greet2/?name={name}", dotenv!("REMOTE_ADDRESS")))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    Ok(result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, greet2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
