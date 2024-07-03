use actix_web::{get, web, Responder};
use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc,
    sync::Mutex,
};

pub fn shared_states_config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .configure(mutable_state_config)
        .service(web::scope("/shared-mutable-state").service(mutable_counter))
        .service(web::scope("/another_app_state").service(add_one_to_another_app_state));
}

pub struct AppState {
    pub app_name: String,
}

#[get("/index")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}

pub struct MutableState {
    pub counter: Mutex<i32>,
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
pub struct AnotherAppState {
    pub worker_local_count: Cell<usize>,
    pub global_count: Arc<AtomicUsize>,
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
