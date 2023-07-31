use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod todolist;
use todolist::services;

struct AppState {
    todolist_data: Mutex<Vec<TodoListData>>
}

// Izinkan struct ini menjadi JSON
#[derive(Deserialize, Serialize, Clone)]
struct TodoListData {
    id: i32,
    date: i64,
    desc: String,
    title: String
}

#[get("/")]
async fn index() -> String {
    "Hello World to Index Function".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let app_data = web::Data::new(AppState {
        todolist_data: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


