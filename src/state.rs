use super::models::Course;
use std::sync::Mutex;

pub struct AppState {
    pub health_checker_response: String,
    pub counter: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}
