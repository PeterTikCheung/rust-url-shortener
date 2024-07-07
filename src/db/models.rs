use super::schema::urls;
use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

//implementations for certain traits, queryable is auto data mapping, insertable is for inserting data into a database table,
//Serialize and Deserialize: These traits are part of the Serde library, which allows you to serialize (convert to a format like JSON) 
//and deserialize (convert from a format like JSON) Rust data structures.
#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "urls"]
pub struct Url {
    pub id: i32,
    pub original_url: String,
    pub short_code: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "urls"]
pub struct NewUrl {
    pub original_url: String,
    pub short_code: String,
}