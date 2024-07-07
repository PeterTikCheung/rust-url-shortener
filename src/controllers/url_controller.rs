use actix_web::{web, HttpResponse, Responder};
use crate::services::url_service;
use crate::db::connection::DbPool;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUrl {
    original_url: String,
}

// Insert the data with the original URL and the short code
pub async fn shorten_url(form: web::Json<CreateUrl>) -> impl Responder {
    match url_service::shorten_url(&form.original_url).await {
        Ok(new_url) => {
            println!("Successfully saved URL with short code: {}", new_url.short_code);
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
    match url_service::get_original_url(&short_code_from_path).await {
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