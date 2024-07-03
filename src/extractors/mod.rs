use actix_web::{get, post, web};
use serde::Deserialize;

pub fn extractors_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/extractors")
            .service(path_extractor)
            .service(path_extractor_2)
            .service(query_extractor)
            .service(json_extractor)
            .service(url_encoded_form_extractor),
    );
}

#[get("/path/{user_id}/{info}")]
async fn path_extractor(path: web::Path<(u32, String)>) -> std::io::Result<String> {
    let (user_id, info) = path.into_inner();
    Ok(format!("User_id: {}, info: {}.", user_id, info))
}

#[derive(Deserialize)]
struct PathParam {
    user_id: u32,
    info: String,
}

#[get("/path_2/{user_id}/{info}")]
async fn path_extractor_2(path_param: web::Path<PathParam>) -> std::io::Result<String> {
    Ok(format!(
        "User_id: {}, info: {}.",
        path_param.user_id, path_param.info
    ))
}

#[derive(Deserialize)]
struct QueryParam {
    username: String,
    id: u32,
}

#[get("/query")]
async fn query_extractor(query: web::Query<QueryParam>) -> String {
    format!("Welcome {}: {}!", query.id, query.username)
}

#[derive(Deserialize)]
struct JsonParam {
    username: String,
    id: u32,
}

#[post("/json")]
async fn json_extractor(json: web::Json<JsonParam>) -> std::io::Result<String> {
    Ok(format!("Welcome {}: {}!", json.id, json.username))
}

#[derive(Deserialize)]
struct FormData {
    username: String,
    id: u32,
}

#[post("/form")]
async fn url_encoded_form_extractor(form: web::Form<FormData>) -> std::io::Result<String> {
    Ok(format!("Welcome {}: {}!", form.id, form.username))
}
