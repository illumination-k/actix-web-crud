use crate::database::Pool;
use crate::models::Post;
use crate::schema::posts;
use actix_web::{get, web, HttpResponse};
use diesel::RunQueryDsl;

#[get("/posts")]
pub async fn index(pool: web::Data<Pool>) -> HttpResponse {
    let conn = pool
        .get()
        .expect("couldn't get driver connection from pool");

    let results: Vec<Post> = web::block(move || posts::table.load(&conn))
        .await
        .map_err(|e| {
            eprintln!("Error: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Error in load posts");

    HttpResponse::Ok().json(results)
}