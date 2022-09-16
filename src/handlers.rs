use super::state::AppState;
use actix_web::{web, HttpResponse};
use std::convert::TryFrom;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_checker_resp = &app_state.health_checker_response;
    let mut vis = app_state.counter.lock().unwrap();
    let resp = format!("{} {} times", health_checker_resp, vis);
    *vis += 1;
    HttpResponse::Ok().json(&resp)
}

use super::db_access;
use super::models::Course;

pub async fn create_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("received new course");
    let course = db_access::create_course(&app_state.db, new_course.into()).await;
    HttpResponse::Ok().json(course)
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
) -> HttpResponse {
    let teacher_id = i32::try_from(params.0).unwrap();
    let courses = db_access::get_courses(&app_state.db, teacher_id).await;
    HttpResponse::Ok().json(courses)
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    let course = db_access::get_course_detail(&app_state.db, teacher_id, course_id).await;
    HttpResponse::Ok().json(course)
}

#[cfg(test)]
mod tests {
    use super::Course;
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn test_create_course() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let share_data = web::Data::new(AppState {
            health_checker_response: "I'm alive!".to_string(),
            counter: Mutex::new(0),
            db: db_pool,
        });

        let course = web::Json(Course {
            id: None,
            teacher_id: 1,
            name: String::from("unit test"),
            time: None,
        });
        let resp = create_course(course, share_data).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_courses_for_teacher() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let share_data = web::Data::new(AppState {
            health_checker_response: "I'm alive!".to_string(),
            counter: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(usize,)> = web::Path::from((1,));
        let resp = get_courses_for_teacher(share_data, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_course_detail() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let share_data = web::Data::new(AppState {
            health_checker_response: "I'm alive!".to_string(),
            counter: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(usize, usize)> = web::Path::from((1, 3));
        let resp = get_course_detail(share_data, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
