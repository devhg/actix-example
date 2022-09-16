use super::models::Course;
use sqlx::postgres::PgPool;

pub async fn get_courses(db: &PgPool, teacher_id: i32) -> Vec<Course> {
    let rows = sqlx::query!(
        r#"select id,teacher_id,name,time from course where teacher_id = $1 "#,
        teacher_id
    )
    .fetch_all(db)
    .await
    .unwrap();

    rows.iter()
        .map(|r| Course {
            id: Some(r.id),
            teacher_id: r.teacher_id,
            name: r.name.clone(),
            time: Some(chrono::NaiveDateTime::from(r.time.unwrap())),
        })
        .collect()
}

pub async fn get_course_detail(db: &PgPool, teacher_id: i32, course_id: i32) -> Course {
    let row = sqlx::query!(
        r#"select id,teacher_id,name,time from course where teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
    .fetch_one(db)
    .await
    .unwrap();

    Course {
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name.clone(),
        time: Some(chrono::NaiveDateTime::from(row.time.unwrap())),
    }
}

pub async fn create_course(db: &PgPool, new_course: Course) -> Course {
    let row = sqlx::query!(
        r#"insert into course(teacher_id, name)
        values($1, $2) returning id, teacher_id, name, time"#,
        new_course.teacher_id,
        new_course.name,
    )
    .fetch_one(db)
    .await
    .unwrap();

    Course {
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name.clone(),
        time: Some(chrono::NaiveDateTime::from(row.time.unwrap())),
    }
}
