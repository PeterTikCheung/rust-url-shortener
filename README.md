#Prerequisite
sudo apt-get install libmysqlclient-dev
Mysql:
CREATE DATABASE url_shortener;

cargo install diesel_cli --no-default-features --features mysql
diesel setup --database-url mysql://username:password@localhost/url_shortener
diesel migration run --database-url mysql://username:password@localhost/url_shortener