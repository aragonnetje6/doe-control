#![warn(clippy::pedantic, clippy::unwrap_used, clippy::nursery)]

use actix_web::{get, post, web};
use askama::Template;
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;

#[derive(Debug, Template)]
#[template(path = "hello_world.html")]
struct HelloWorldTemplate<'a> {
    text: &'a str,
}

impl<'a> HelloWorldTemplate<'a> {
    const fn new(text: &'a str) -> Self {
        Self { text }
    }
}

#[derive(Debug, Template)]
#[template(path = "base.html", escape = "none")]
struct BaseTemplate<'a, T: Template> {
    title: &'a str,
    body: T,
}

impl<'a, T: Template> BaseTemplate<'a, T> {
    const fn new(title: &'a str, body: T) -> Self {
        Self { title, body }
    }
}

#[get("/")]
async fn hello_world() -> web::Html {
    web::Html::new(
        BaseTemplate::new("DoeControl", HelloWorldTemplate::new("Hello world!"))
            .render()
            .expect("infallible"),
    )
}

#[derive(Debug, Template)]
#[template(path = "login.html")]
struct LoginTemplate;

#[get("/login")]
async fn login() -> web::Html {
    web::Html::new(
        BaseTemplate::new("DoeControl Login", LoginTemplate)
            .render()
            .expect("infallible"),
    )
}

#[derive(Debug, Deserialize)]
struct Login {
    password: String,
}

#[post("/login")]
async fn login_post(password: web::Form<Login>) -> web::Html {
    if password.password == "hello" {
        web::Html::new("Well done!")
    } else {
        web::Html::new("Wrong password")
    }
}

#[derive(Deserialize)]
struct Name {
    name: String,
}

#[derive(Debug, Template)]
#[template(path = "greet2.html")]
struct Greet2Template<'a> {
    name: &'a str,
}

impl<'a> Greet2Template<'a> {
    const fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[get("/greet2/")]
async fn greet2(name: web::Query<Name>) -> web::Html {
    tracing::info!("greeting {}", name.name);
    web::Html::new(
        BaseTemplate::new("Greet2", Greet2Template::new(&name.name))
            .render()
            .expect("infallible"),
    )
}

#[allow(clippy::unused_async)]
#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.service(hello_world)
            .service(greet2)
            .service(login)
            .service(login_post)
            .service(actix_files::Files::new("/assets", "./assets").show_files_listing())
            .app_data(web::Data::new(pool));
    };

    Ok(config.into())
}
