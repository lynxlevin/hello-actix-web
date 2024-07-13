use actix_web::{get, post, web, Responder, Result};
use serde::Deserialize;

use crate::diesel_code::{create_todo, establish_connection, models::Todo};
use diesel::prelude::*;

pub fn todo_app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/todo").service(get_all).service(create));
}

#[get("")]
async fn get_all() -> Result<impl Responder> {
    use crate::diesel_code::schema::todos::dsl::*;

    let connection = &mut establish_connection();
    let results = todos
        .select(Todo::as_select())
        .load(connection)
        .expect("Error loading todos");

    Ok(web::Json(results))
}

#[derive(Deserialize)]
struct NewTodoRequest {
    title: String,
    description: String,
}

#[post("")]
async fn create(json: web::Json<NewTodoRequest>) -> Result<impl Responder> {
    let connection = &mut establish_connection();

    let todo = create_todo(connection, &json.title, &json.description);

    Ok(web::Json(todo))
}
