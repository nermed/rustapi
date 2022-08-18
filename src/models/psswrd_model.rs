use crate::services::psswrd;
use diesel::Queryable;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
#[derive(sqlx::FromRow, Debug, Clone)]
// #[serde(crate = "rocket::serde")]
// #[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Password {
    pub id: u64,
    pub full_name: String,
    pub password: String,
    pub date_created: String,
}
