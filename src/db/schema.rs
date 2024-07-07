use diesel::table;

table! {
    urls (id) {
        id -> Integer,
        original_url -> Varchar,
        short_code -> Varchar,
    }
}