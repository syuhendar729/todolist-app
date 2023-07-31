use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{AppState, TodoListData};
use super::models::{CreateData, UpdateData};

#[get("/todolist")]
async fn get_data(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todolist_data.lock().unwrap().to_vec())
}

#[post("/todolist")]
async fn create_data(data: web::Data<AppState>, body_json: web::Json<CreateData>) -> impl Responder {
    let mut todolist_data = data.todolist_data.lock().unwrap();
    let mut max_id = 0;

    for i in 0..todolist_data.len() {
        if todolist_data[i].id > max_id {
            max_id = todolist_data[i].id;
        }
    }
    // max_id akan menjadi id terakhir dari data, dan nanti ditambah 1
    // data.len() = berapa data bukan index
    // data[0].id = 0
    // data[1].id = 1
    // data[2].id = 2
    todolist_data.push(TodoListData {
        id: max_id + 1,
        title: body_json.title.clone(),
        desc: body_json.desc.clone(),
        date: body_json.date.clone()
    });

    HttpResponse::Ok().json(todolist_data.to_vec())
}


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


