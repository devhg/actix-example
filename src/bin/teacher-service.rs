use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let share_data = web::Data::new(AppState {
        health_checker_response: "I'm alive!".to_string(),
        counter: Mutex::new(0),
        courses: Mutex::new(vec![]),
    });

    let app = move || {
        App::new()
            .app_data(share_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
