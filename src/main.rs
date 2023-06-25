use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello world! html</h1>")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("<div style='color: red;'>Hey there!!!!")
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
            .route("/hey", web::get().to(manual_hello))
            .service(
                web::scope("/hop").service(web::scope("/step").route("/jump", web::get().to(jump))),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
