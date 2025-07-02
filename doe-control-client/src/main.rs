#![warn(clippy::pedantic, clippy::unwrap_used, clippy::nursery)]

mod app;

use app::App;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    });
}
