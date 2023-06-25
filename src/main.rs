use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

struct AppState {
    app_name: String,
}

#[derive(Serialize)]
struct Man {
    name: String,
    born_in: isize,
    langs: Vec<String>,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
    HttpResponse::Ok().body(format!(
        "<h1 style='color: red;'>Hello world! html from {app_name}</h1>"
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
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("my-first-actix-web"),
            }))
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
