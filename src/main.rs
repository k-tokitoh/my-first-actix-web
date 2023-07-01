use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::sync::Mutex;

struct AppState {
    app_name: String,
    access_counter: Mutex<i32>,
}

#[derive(Serialize)]
struct Man {
    name: String,
    born_in: isize,
    langs: Vec<String>,
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

async fn who() -> impl Responder {
    let me = Man {
        name: String::from("k-tokitoh"),
        born_in: 1979,
        langs: vec![String::from("rust"), String::from("typescript")],
    };

    HttpResponse::Ok().json(me)
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

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(hello)
            .service(echo)
            .route("/who", web::get().to(who))
            .service(
                web::scope("/hop").service(web::scope("/step").route("/jump", web::get().to(jump))),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
