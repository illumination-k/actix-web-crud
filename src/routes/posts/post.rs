use crate::database::{Pool, PooledPgConnection};
use crate::models::NewPost;
use crate::schema::posts;
use actix_web::{post, web, HttpResponse};
use anyhow::Result;
use diesel::prelude::*;

fn add_post(conn: &PooledPgConnection, new_post: &NewPost) -> Result<()> {
    diesel::insert_into(posts::table)
        .values(new_post)
        .execute(conn)?;
    Ok(())
}

#[post("/posts")]
pub async fn index(pool: web::Data<Pool>, form: web::Json<NewPost>) -> HttpResponse {
    let conn = pool
        .get()
        .expect("couldn't get driver connection from pool");

    let new_post = NewPost {
        title: form.title.clone(),
        body: form.body.clone(),
    };

    web::block(move || add_post(&conn, &new_post))
        .await
        .map_err(|e| {
            eprintln!("ERROR: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .expect("Error in add post");

    eprintln!("Accecpted!");

    HttpResponse::Created().body("Created!")
}
