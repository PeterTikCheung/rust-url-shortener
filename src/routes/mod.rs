pub mod url_routes;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .route("shorten", web::post().to(url_routes::shorten_url))
        .route("/{short_code}", web::get().to(url_routes::redirect))
        .route("/short_code/test_redirection", web::get().to(url_routes::test_redirection));
}