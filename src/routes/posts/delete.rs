use crate::database::{Pool, PooledPgConnection};
use crate::schema::posts;
use actix_web::{delete, web, HttpResponse};
use anyhow::Result;
use diesel::prelude::*;

fn delete(conn: &PooledPgConnection, id: i32) -> Result<()> {
    diesel::delete(posts::table.find(id)).execute(conn)?;
    Ok(())
}

#[delete("/posts/{post_id}")]
pub async fn index(pool: web::Data<Pool>, post_id: web::Path<i32>) -> HttpResponse {
    let conn = pool
        .get()
        .expect("couldn't get driver connection from pool");

    let id = post_id.to_owned();

    web::block(move || delete(&conn, id))
        .await
        .expect("Error in delete");

    HttpResponse::Ok().body(format!("Delete: {}", id))
}
