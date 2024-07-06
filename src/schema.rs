// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Integer,
        #[max_length = 255]
        original_url -> Varchar,
        #[max_length = 8]
        short_code -> Varchar,
    }
}
