# Rust-url-shortener

This project demonstrates a way to use rust define a url with short code and save it to database. After that, the user can call another redirect api to enter the short code in the path and redirect to the url according to the data saved in the db.

"actix" for request manipulation.

"diesel" for Mysql ORM.
## Prerequisite
1. this project is using mysql
```shell
sudo apt-get install libmysqlclient-dev
```
2. mysql shell
```shell
CREATE DATABASE url_shortener;
```
3. Setup diesel and create table
```shell
cargo install diesel_cli --no-default-features --features mysql
diesel setup --database-url mysql://username:password@localhost/url_shortener
diesel migration run --database-url mysql://username:password@localhost/url_shortener
```
4. Create a .env file in the root directory with the mysql db url, below is the example:
```shell
DATABASE_URL = "mysql://root:root@localhost:3306/url_shortener"
```

## Features
Please see the routes in /src/routes directory
- POST "shorten" api, which is used to define a path and generate a short code for it. Afterthat, insert them into the db table. Below is the request body:
```shell
{
    "original_url": "http://127.0.0.1:8080/short_code/test_redirection"
}
```


- GET "/{short_code}" api, redirect to the url found in db related to the path variable {short_code}

- GET "/short_code/test_redirection" api, api for testing the redirection.

## Run the application locally
Run the application
```bash
  cargo run
```