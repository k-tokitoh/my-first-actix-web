use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

pub fn scope() -> actix_web::Scope {
    web::scope("tasks").route("/", web::get().to(index))
}

#[derive(Serialize)]
struct Task {
    id: u32,
    description: String,
}

async fn index() -> impl Responder {
    web::scope("");
    let lunch = Task {
        id: 0,
        description: String::from("have lunch."),
    };
    let bird = Task {
        id: 1,
        description: String::from("watch bird."),
    };
    let tasks = vec![lunch, bird];
    HttpResponse::Ok().json(tasks)
}
