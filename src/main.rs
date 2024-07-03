use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
    sync::Mutex,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

struct AppState {
    app_name: String,
}

#[get("/index")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

struct MutableState {
    counter: Mutex<i32>,
}

#[get("/counter")]
async fn mutable_counter(data: web::Data<MutableState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {counter}")
}

fn mutable_state_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/shared-mutable-state2").service(mutable_counter));
}

#[derive(Clone)]
struct AnotherAppState {
    worker_local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
}

#[get("/add")]
async fn add_one_to_another_app_state(data: web::Data<AnotherAppState>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);

    let worker_local_count = data.worker_local_count.get();
    data.worker_local_count.set(worker_local_count + 1);

    format!(
        "global_count: {}\nworker_local_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.worker_local_count.get(),
    )
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(MutableState {
        // This shares all data across workers.
        counter: Mutex::new(0),
    });
    let another_counter = AnotherAppState {
        // This can control which data to share between workers.
        worker_local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };

    let json_config = web::JsonConfig::default().limit(4096); // This limits the payload below 4KB.;

    // move is needed to use counter. Maybe to share the state across threads?
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .app_data(counter.clone())
            .app_data(web::Data::new(another_counter.clone()))
            .app_data(json_config.clone())
            .service(index)
            .service(hello)
            .service(web::scope("/app").service(echo))
            .route("/hey", web::get().to(manual_hello))
            .service(web::scope("/shared-mutable-state").service(mutable_counter))
            .service(web::scope("/another_app_state").service(add_one_to_another_app_state))
            .configure(mutable_state_config)
            .service(
                web::scope("/extractors")
                    .service(path_extractor)
                    .service(path_extractor_2)
                    .service(query_extractor)
                    .service(json_extractor)
                    .service(url_encoded_form_extractor),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
