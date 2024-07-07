//ensures that any macros defined in the imported crate can be used
#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use nanoid::nanoid;
use serde::Deserialize;
use std::env;
use std::sync::Arc;

mod db;

use db::models::{NewUrl, Url};
use db::schema::urls;

#[derive(Deserialize)]
struct CreateUrl {
    original_url: String,
}

//Arc ensure thread-safe reference counting
//insert the data with the original url and the short code
async fn shorten_url(conn: &mut MysqlConnection, form: web::Json<CreateUrl>) -> impl Responder {
    use self::urls::dsl::*;

    //short code generation
    let new_short_code = nanoid!(8);
    let new_url = NewUrl {
        original_url: form.original_url.clone(),
        short_code: new_short_code.clone(),
    };

    diesel::insert_into(urls)
        .values(&new_url)
        .execute(conn)
        .expect("Error saving new URL");

    HttpResponse::Ok().json(new_url)
}


//use the short code to redirect to the original url
async fn redirect(conn: &mut MysqlConnection, short_code: String) -> impl Responder {
    use self::urls::dsl::*;

    let result: Option<Url> = urls
        .filter(short_code.eq(&short_code))
        .first::<Url>(conn)
        .optional()
        .expect("Error loading URL");

    match result {
        Some(url) => HttpResponse::Found()
            .header("Location", url.original_url)
            .finish(),
        None => HttpResponse::NotFound().finish(),
    }
}
