// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::pedantic, clippy::unwrap_used, clippy::nursery)]

fn main() {
    doe_control_client_lib::run()
}
