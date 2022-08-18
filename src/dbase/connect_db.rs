// use mysql::*;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};
use std::time::Duration;

pub type Db = Pool<MySql>;

const PG_HOST: &str = "localhost";
const PG_ROOT_DB: &str = "psswrd_manager";
const PG_ROOT_USER: &str = "root";
const PG_ROOT_PWD: &str = "";

// pub fn connect () -> Pool {
//     // let url = "mysql://root:pass@localhost:3306/rustapi-test";
//     let opts = OptsBuilder::new()

//         .ip_or_hostname(Some("localhost"))

//         .user(Some("root"))

//         .pass(Some(""))

//         .db_name(Some("psswrd_manager"));

//     let pool = Pool::new(opts).unwrap();

//     pool
// }

async fn new_db_pool(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_con: u32,
) -> Result<Db, sqlx::Error> {
    let con_string = format!("mysql://{}:{}@{}/{}", user, pwd, host, db);
    MySqlPoolOptions::new()
        .max_connections(max_con)
        .acquire_timeout(Duration::from_millis(500)) // Needs to find replacement
        .connect(&con_string)
        .await
}

pub async fn connect() -> Result<Db, sqlx::Error> {
    new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await
}

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

// pub fn connect() -> MysqlConnection {
//     dotenv().ok();

//     let database_url = env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     MysqlConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }
