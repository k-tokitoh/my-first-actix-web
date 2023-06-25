use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("<h1 style='color: red;'>Hello world! html</h1>")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn who() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"name":"takashi","born_in":1979,"lang":["rust","typescript"]}"#)
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
