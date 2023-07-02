use crate::database::DbPool;
use crate::schema::tasks::dsl::*;
use actix_web::{error, web, HttpResponse, Responder};
use serde::Serialize;
extern crate diesel;
use self::diesel::prelude::*;

pub fn scope() -> actix_web::Scope {
    web::scope("/tasks")
        .route("/", web::get().to(index))
        .service(
            web::resource("{id}")
                .name("task")
                .route(web::get().to(read)),
        )
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

async fn read(
    pool: web::Data<DbPool>,
    task_id: web::Path<(i32,)>,
) -> actix_web::Result<impl Responder> {
    let (task_id,) = task_id.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        get_task(&mut conn, task_id)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

fn get_task(conn: &mut SqliteConnection, task_id: i32) -> diesel::QueryResult<Vec<Task>> {
    let result = tasks
        .filter(id.eq(task_id))
        .load::<Task>(conn)
        .expect("Error loading person that was just inserted");
    Ok(result)
}
