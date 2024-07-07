use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use nanoid::nanoid;
use serde::Deserialize;
use crate::db::models::{NewUrl, Url};
use crate::db::schema::urls::dsl::*;
use crate::db::connection::establish_connection;

#[derive(Deserialize)]
pub struct CreateUrl {
    original_url: String,
}

// Insert the data with the original URL and the short code
pub async fn shorten_url(form: web::Json<CreateUrl>) -> impl Responder {
    let mut conn = establish_connection();

    // Short code generation
    let new_short_code = nanoid!(8);
    let new_url = NewUrl {
        original_url: form.original_url.clone(),
        short_code: new_short_code.clone(),
    };

    let result = diesel::insert_into(urls)
        .values(&new_url)
        .execute(&mut conn);

    match result {
        Ok(_) => {
            println!("Successfully saved URL with short code: {}", new_short_code);
            HttpResponse::Ok().json(new_url)
        },
        Err(e) => {
            println!("Error saving URL: {}", e);
            HttpResponse::InternalServerError().body("Error saving URL")
        }
    }
}

// Use the short code to redirect to the original URL
pub async fn redirect(short_code_from_path: web::Path<String>) -> impl Responder {
    let mut conn = establish_connection();
    
    println!("Looking up short code: {}", short_code_from_path);

    let result: Result<Option<Url>, diesel::result::Error> = urls
        .filter(short_code.eq(short_code_from_path.as_str()))
        .first::<Url>(&mut conn)
        .optional();

    match result {
        Ok(Some(url)) => {
            println!("Found URL: {}", url.original_url);
            HttpResponse::Found()
                .append_header(("Location", url.original_url))
                .finish()
        },
        Ok(None) => {
            println!("No URL found for short code: {}", short_code_from_path);
            HttpResponse::NotFound().finish()
        },
        Err(e) => {
            println!("Error loading URL: {}", e);
            HttpResponse::InternalServerError().body("Error loading URL")
        }
    }
}

// Test endpoint to be redirected to
pub async fn test_redirection() -> impl Responder {
    println!("Redirection successful!");
    HttpResponse::Ok().body("Redirection successful!")
}