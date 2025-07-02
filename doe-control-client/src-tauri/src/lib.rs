use tauri_plugin_http::reqwest;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
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
    let result = reqwest::get(format!(
        "https://doe-control-server-fvte.shuttle.app/greet2/?name={name}"
    ))
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
