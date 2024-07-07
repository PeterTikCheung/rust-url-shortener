use crate::db::models::{NewUrl, Url};
use crate::db::schema::urls::dsl::*;
use crate::db::connection::establish_connection;
use diesel::prelude::*;
use diesel::result::Error;

pub async fn shorten_url(original_url: &str) -> Result<NewUrl, Error> {
    let mut conn = establish_connection();
    // Short code generation
    let new_short_code = nanoid::nanoid!(8);
    let new_url = NewUrl {
        original_url: original_url.to_string(),
        short_code: new_short_code.clone(),
    };

    diesel::insert_into(urls)
        .values(&new_url)
        .execute(&mut conn)?;

    Ok(new_url)
}

pub async fn get_original_url(short_code_str: &str) -> Result<Option<Url>, Error> {
    let mut conn = establish_connection();
    let result: Result<Option<Url>, Error> = urls
        .filter(short_code.eq(short_code_str))
        .first::<Url>(&mut conn)
        .optional();

    result
}