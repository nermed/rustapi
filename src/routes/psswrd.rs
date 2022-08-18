use rocket::serde::{Deserialize, Serialize};
use crate::services::psswrd;
use rocket::serde::json::Json;
use crate::dbase::connect_db::connect;
use std::sync::Arc;

// #[derive(Serialize, Deserialize, Debug)]
#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Password {
    pub id: u64,
    pub full_name: String,
    pub password: String,
    pub date_created: String,
}

impl Password {
    fn new(id: u64, full_name: String, password: String, date_created: String) -> Password {
        Password{
            id,
            full_name,
            password,
            date_created
        }
    }
}

#[get("/")]
pub async fn get_allpass() -> &'static str {
    // Json(psswrd::get_allpass())
    let db = connect().await.expect("Cannot init db");
	// let db = Arc::new(db);
    let resp = psswrd::get_allpass(&db);
    // println!("{:#?}", resp);
    "hello list"
}

#[get("/<id_input>")]
pub fn get_onepass(id_input: i32) -> &'static str {
    // Json(psswrd::get_onepass(id_input))
    psswrd::get_onepass(id_input);
    println!("{}", id_input);
    "Hello"
}

#[get("/old")]
pub fn get_oldpass() -> &'static str {
    // Json(psswrd::get_oldpass())
    "hello"
}