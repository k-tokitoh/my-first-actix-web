use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Man {
    name: String,
    born_in: isize,
    langs: Vec<String>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("<h1 style='color: red;'>Hello world! html</h1>")
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
