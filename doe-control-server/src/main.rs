use actix_web::{get, web};
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[derive(Deserialize)]
struct Name {
    name: String,
}

#[get("/greet2/")]
async fn greet2(name: web::Query<Name>) -> String {
    tracing::info!("greeting {}", name.name);
    format!("Hello from Miss Grace' server, {}!", name.name)
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(hello_world).service(greet2);
    };

    Ok(config.into())
}
