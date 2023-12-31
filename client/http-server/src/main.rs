use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod database;
mod schema;
mod tasks;
mod who;

struct AppState {
    app_name: String,
    access_counter: Mutex<i32>,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let mut access_counter = data.access_counter.lock().unwrap();
    *access_counter += 1;

    HttpResponse::Ok().body(format!(
        r#"
            <h1 style='color: red;'>Hello world! html from {app_name}</h1>
            <div>access_counter: {access_counter}</div>
        "#
    ))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn jump() -> impl Responder {
    "jumped."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        app_name: String::from("my-first-actix-web"),
        access_counter: Mutex::new(0),
    });

    let pool = database::get_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(echo)
            .route("/who", web::get().to(who::handler))
            .service(
                web::scope("/hop").service(web::scope("/step").route("/jump", web::get().to(jump))),
            )
            .service(tasks::scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
