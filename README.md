# TodoList App with Actix 
## Project Backend Rest API dengan framework Rust Actix Web

### Penjelasan Code:

#### File `main.rs`

Mengimport library yang dibutuhkan ke file utama

`main.rs`
```rust
use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod todolist;
use todolist::services;

```

Lalu membuat `struct` sesuai dengan tipe data yang diperlukan sebagai berikut

`main.rs`
```rust
struct AppState {
    todolist_data: Mutex<Vec<TodoListData>>
}

#[derive(Deserialize, Serialize, Clone)]
struct TodoListData {
    id: i32,
    date: i64,
    desc: String,
    title: String
}
```

Kode selanjutnya adalah membuat fungsi utama untuk menjalankan Actix Web itu sendiri

```rust
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
```

#### File `todolist/mod.rs`

Tujuan `mod.rs` hanya untuk mengexport modul(file) yang ada di dalam folder `todolist` agar bisa digunakan di `main.rs`

```rust
pub mod services;
mod models;
```

Tujuan dibuatnya `models.rs` adalah untuk membuat model data 

`CreateData` | untuk membuat data |
`UpdateData` | untuk mengedit data |

| Struct        | Tipe data   | Description            |
|---------------|-------------|------------------------|
| `CreateData`  | `title`: string, `desc`: string, `date`: i64  | untuk membuat data     |
| `UpdateData`  | `title`: string, `desc`: string               | untuk mengedit data    |

#### File `todolist/models.rs`

```rust
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateData {
    pub title: String,
    pub desc: String,
    pub date: i64
}

#[derive(Deserialize, Clone)]
pub struct UpdateData {
    pub title: String,
    pub desc: String,
}

```

#### File `todolist/services.rs`

Mengimport module yang diperlukan untuk menggunakan Actix Web dan Model data dari `models.rs`

```rust
use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{AppState, TodoListData};
use super::models::{CreateData, UpdateData};
```

Metode `GET` untuk mengambil data

```rust
#[get("/todolist")]
async fn get_data(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todolist_data.lock().unwrap().to_vec())
}
```

Metode `POST` untuk menambah data

```rust
#[post("/todolist")]
async fn create_data(data: web::Data<AppState>, body_json: web::Json<CreateData>) -> impl Responder {
    let mut todolist_data = data.todolist_data.lock().unwrap();
    let mut max_id = 0;

    for i in 0..todolist_data.len() {
        if todolist_data[i].id > max_id {
            max_id = todolist_data[i].id;
        }
    }
    todolist_data.push(TodoListData {
        id: max_id + 1,
        title: body_json.title.clone(),
        desc: body_json.desc.clone(),
        date: body_json.date.clone()
    });

    HttpResponse::Ok().json(todolist_data.to_vec())
}
```

Metode `PUT` untuk mengedit atau mengupdate data

```rust
#[put("/todolist/{id}")]
async fn update_data(data: web::Data<AppState>, id: web::Path<i32>, body_json: web::Json<UpdateData>) -> impl Responder {
    let mut todolist_data = data.todolist_data.lock().unwrap();
    let id = id.into_inner();

    for i in 0..todolist_data.len() {
        if todolist_data[i].id == id {
            todolist_data[i].title = body_json.title.clone();
            todolist_data[i].desc = body_json.desc.clone();
            break;
        }
    }

    HttpResponse::Ok().json(todolist_data.to_vec())
}

```

Metode `DELETE` untuk menghapus data

```rust

#[delete("/todolist/{id}")]
async fn delete_data(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let mut todolist_data = data.todolist_data.lock().unwrap();
    let id = id.into_inner();
    
    *todolist_data = todolist_data.to_vec().into_iter().filter(|x| x.id != id).collect();

    HttpResponse::Ok().json(todolist_data.to_vec())
}

// Ini mirip dengan app.use() di nodejs
// Agar route API nya melewati semua service atau controller diatas
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_data)
        .service(create_data)
        .service(update_data)
        .service(delete_data);
}

```
