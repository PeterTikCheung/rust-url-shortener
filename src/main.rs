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


fn main() {
    println!("Hello, world!");
}
