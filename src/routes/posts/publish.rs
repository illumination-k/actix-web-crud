use crate::database::{Pool, PooledPgConnection};
use crate::models::Post;
use crate::schema::posts;
use actix_web::{put, web, HttpResponse};
use anyhow::Result;
use diesel::prelude::*;

fn published(conn: &PooledPgConnection, id: i32) -> Result<Post> {
    let post: Post = diesel::update(posts::table.find(id))
        .set(posts::published.eq(true))
        .get_result(conn)?;

    Ok(post)
}

#[put("/posts/publish/{post_id}")]
pub async fn index(pool: web::Data<Pool>, post_id: web::Path<i32>) -> HttpResponse {
    let conn = pool
        .get()
        .expect("couldn't get driver connection from pool");

    let id = post_id.to_owned();

    let post: Post = web::block(move || published(&conn, id))
        .await
        .expect("Error in published");

    HttpResponse::Ok().body(format!("Published:\n title: {}", post.title))
}
