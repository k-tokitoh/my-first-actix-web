use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Man {
    name: String,
    born_in: isize,
    langs: Vec<String>,
}

pub async fn handler() -> impl Responder {
    let me = Man {
        name: String::from("k-tokitoh"),
        born_in: 1979,
        langs: vec![String::from("rust"), String::from("typescript")],
    };

    HttpResponse::Ok().json(me)
}
