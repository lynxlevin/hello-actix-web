mod extractors;
mod shared_states;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{cell::Cell, sync::atomic::AtomicUsize, sync::Arc, sync::Mutex};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(shared_states::MutableState {
        // This shares all data across workers.
        counter: Mutex::new(0),
    });
    let another_counter = shared_states::AnotherAppState {
        // This can control which data to share between workers.
        worker_local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0)),
    };
    let json_config = web::JsonConfig::default().limit(4096); // This limits the payload below 4KB.;

    // move is needed to use counter. Maybe to share the state across threads?
    HttpServer::new(move || {
        App::new()
            // These app_datas are not shared between workers if these are written inside mod.rs
            .app_data(web::Data::new(shared_states::AppState {
                app_name: String::from("Actix Web"),
            }))
            .app_data(counter.clone())
            .app_data(web::Data::new(another_counter.clone()))
            .app_data(json_config.clone())
            .service(hello)
            .service(web::scope("/app").service(echo))
            .route("/hey", web::get().to(manual_hello))
            .configure(shared_states::shared_states_config)
            .configure(extractors::extractors_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
