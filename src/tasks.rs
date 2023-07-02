use crate::database::DbPool;
use crate::schema::tasks::dsl::*;
use actix_web::{error, web, HttpResponse, Responder};
use serde::Serialize;
extern crate diesel;
use self::diesel::prelude::*;

pub fn scope() -> actix_web::Scope {
    web::scope("tasks").route("/", web::get().to(index))
}

#[derive(Serialize, Queryable)]
pub struct Task {
    pub id: Option<i32>,
    pub body: String,
}

async fn index(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        get_tasks(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

fn get_tasks(conn: &mut SqliteConnection) -> diesel::QueryResult<Vec<Task>> {
    let result = tasks
        .load::<Task>(conn)
        .expect("Error loading person that was just inserted");
    Ok(result)
}
