use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_checker_resp = &app_state.health_checker_response;
    let mut vis = app_state.counter.lock().unwrap();
    let resp = format!("{} {} times", health_checker_resp, vis);
    *vis += 1;
    HttpResponse::Ok().json(&resp)
}

use super::models::Course;
use chrono::Utc;

pub async fn create_course_handler(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("received new course");
    let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .collect::<Vec<Course>>()
        .len();

    let new_course = Course {
        id: Some(course_count + 1),
        name: new_course.name.clone(),
        teacher_id: new_course.teacher_id,
        time: Some(Utc::now().naive_utc()),
    };

    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("course created")
}

pub async fn get_course_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> HttpResponse {
    let teacher_id = params.0;
    println!("{}", teacher_id);
    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == teacher_id)
        .collect::<Vec<Course>>();

    if filtered_courses.len() > 0 {
        HttpResponse::Ok().json(filtered_courses)
    } else {
        HttpResponse::Ok().json("no courses found")
    }
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let (teacher_id, course_id) = params.0;
    println!("{}, {}", teacher_id, course_id);
    let selected_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|course| course.teacher_id == teacher_id && course.id == Some(course_id))
        .ok_or("course not found");

    if let Ok(course) = selected_courses {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("course not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, App};
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn test_create_course_handler() {
        let course = web::Json(Course {
            id: None,
            name: "Rust".to_string(),
            teacher_id: 1,
            time: None,
        });

        let app_state = web::Data::new(AppState {
            health_checker_response: "create_course_handler".to_string(),
            counter: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let resp = create_course_handler(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_course_for_teacher() {
        let app_state = web::Data::new(AppState {
            health_checker_response: "create_course_handler".to_string(),
            counter: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let params: web::Path<usize> = web::Path::from(1);

        let resp = get_course_for_teacher(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_course_detail() {
        let app_state = web::Data::new(AppState {
            health_checker_response: "create_course_handler".to_string(),
            counter: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let params: web::Path<(usize, usize)> = web::Path::from((1, 1));

        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
