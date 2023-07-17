use crate::database::DbPool;
use crate::schema::tasks::dsl::{id, tasks};
use actix_web::{error, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
extern crate diesel;
use self::diesel::prelude::*;

pub fn scope() -> actix_web::Scope {
    web::scope("/tasks")
        .service(
            web::resource("/")
                .name("tasks")
                .route(web::get().to(index))
                .route(web::put().to(create)),
        )
        .service(
            web::resource("{id}")
                .name("task")
                .route(web::get().to(read))
                .route(web::put().to(update))
                .route(web::delete().to(delete)),
        )
}

// dieselがSeriablizable/Deserializableというのを提供してそうだけど、シンプルに捉えるためにserdeを利用する
#[derive(Serialize, Deserialize, Queryable, Selectable)]
// 続く2行でスキーマのとの整合性を確かめてくれるらしい
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub body: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask {
    pub body: String,
}

#[derive(Deserialize)]
struct UpdateRequestParams {
    pub body: String,
}

// view
async fn index(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().map_err(error::ErrorInternalServerError)?;
    let result = web::block(move || get_tasks(&mut conn))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

// service
fn get_tasks(conn: &mut SqliteConnection) -> diesel::QueryResult<Vec<Task>> {
    let result = tasks.load(conn)?;
    // .load::<Task>(conn)  こっちでもOK
    // あるいはSelectableをderiveしてas_select()とかつかってもいいらしい。よくわかってない
    Ok(result)
}

// view
async fn read(
    pool: web::Data<DbPool>,
    task_id: web::Path<(i32,)>,
) -> actix_web::Result<impl Responder> {
    let (task_id,) = task_id.into_inner();

    let mut conn = pool.get().map_err(error::ErrorInternalServerError)?;
    let result = web::block(move || get_task(&mut conn, task_id))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

// service
fn get_task(conn: &mut SqliteConnection, task_id: i32) -> diesel::QueryResult<Task> {
    let result = tasks.filter(id.eq(task_id)).first::<Task>(conn)?;
    Ok(result)
}

// view
async fn create(
    pool: web::Data<DbPool>,
    form: web::Json<NewTask>,
) -> actix_web::Result<impl Responder> {
    let new_task = NewTask {
        body: form.body.clone(),
    };
    let mut conn = pool.get().map_err(error::ErrorInternalServerError)?;
    let result = web::block(move || create_task(&mut conn, &new_task))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

// service
pub fn create_task(conn: &mut SqliteConnection, new_task: &NewTask) -> diesel::QueryResult<Task> {
    let result = diesel::insert_into(crate::schema::tasks::table)
        .values(new_task)
        .returning(Task::as_returning()) // as_returning() はSelectableによって利用可能になっているらしい
        .get_result(conn)?;
    Ok(result)
}

// view
async fn update(
    pool: web::Data<DbPool>,
    task_id: web::Path<(i32,)>,
    form: web::Json<UpdateRequestParams>,
) -> actix_web::Result<impl Responder> {
    let (task_id,) = task_id.into_inner();

    let mut conn = pool.get().map_err(error::ErrorInternalServerError)?;
    let result = web::block(move || update_task(&mut conn, task_id, &form.body))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

// service
pub fn update_task(
    conn: &mut SqliteConnection,
    task_id: i32,
    body: &String,
) -> diesel::QueryResult<Task> {
    let task: Task = diesel::update(crate::schema::tasks::table.find(task_id))
        .set(crate::schema::tasks::body.eq(body))
        .get_result(conn)?;

    Ok(task)
}

// view
async fn delete(
    pool: web::Data<DbPool>,
    task_id: web::Path<(i32,)>,
) -> actix_web::Result<impl Responder> {
    let (task_id,) = task_id.into_inner();

    let mut conn = pool.get().map_err(error::ErrorInternalServerError)?;
    let result = web::block(move || delete_task(&mut conn, task_id))
        .await?
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

// service
pub fn delete_task(conn: &mut SqliteConnection, task_id: i32) -> diesel::QueryResult<Task> {
    let task: Task = diesel::delete(crate::schema::tasks::table.find(task_id)).get_result(conn)?;

    Ok(task)
}
