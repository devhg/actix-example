// mod db_access;

// use chrono::NaiveDateTime;
// use dotenv::dotenv;
// use sqlx::postgres::PgPoolOptions;
// use std::env;
// use std::io;
// use std::vec;

// #[derive(Debug)]
// pub struct Course {
//     pub id: i32,
//     pub teacher_id: i32,
//     pub name: String,
//     pub time: Option<NaiveDateTime>,
// }

// #[actix_rt::main]
// async fn main() -> io::Result<()> {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("error .env");
//     println!("{}", database_url);
//     let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
//     let course_rows = sqlx::query!(
//         r#"select id,teacher_id,name,time from course where id = $1 "#,
//         1
//     )
//     .fetch_all(&db_pool)
//     .await
//     .unwrap();

//     let mut course_list = vec![];
//     for row in course_rows {
//         course_list.push(Course {
//             id: row.id,
//             teacher_id: row.teacher_id,
//             name: row.name,
//             time: Some(chrono::NaiveDateTime::from(row.time.unwrap())),
//         })
//     }
//     println!("{:?}", course_list);
//     Ok(())
// }

fn main() {}
