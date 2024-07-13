use actix_web::{post, web, Responder, Result};
use serde::Deserialize;

use crate::diesel_code::{create_todo, establish_connection};

pub fn todo_app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/todo").service(create));
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
