use mysql::prelude::*;
use sqlx;
// use craconn::Db;

// import our Password object from the routes/psswrd module
use crate::dbase::Db;
use crate::models::psswrd_model::Password;

// pub async fn get_allpass(db: &Db) -> Result<Vec<Password>, sqlx::Error> {
//     let sb = sqlb::select().table("PASSWORD_MANAGER").columns(&["id", "full_name", "password", "date_created"]).order_by("!id");

//     // execute the query
//     let pass = sb.fetch_all(db).await?;

//     Ok(pass)
// }

pub async fn get_allpass(db: &Db) -> Result<Vec<Password>, sqlx::Error> {
    println!("new");
    let sql = "select ID as id, FULL_NAME as full_name, PASSWORD as password, DATE_CREATED from PASSWORD_MANAGER";
    let query = sqlx::query_as(&sql);
    let all_pass = query.fetch_all(db).await?;
    for pass in &all_pass {
        println!("{:?}", pass);
    }
    // println!("{:#?}", all_pass);
    Ok(all_pass)
    // pub fn get_allpass() -> Vec<Password> {
    //     let mut conn = connect_db::connect().get_conn().unwrap();

    //     let res = conn
    //         .query_map(
    //             "select ID as id, FULL_NAME as full_name, PASSWORD as password, DATE_CREATED from PASSWORD_MANAGER",
    //             |(id, full_name, password, date_created)| Password {
    //                 id,
    //                 full_name,
    //                 password,
    //                 date_created,
    //             },
    //         )
    //         .unwrap();
    //     res
}

pub fn get_onepass(id_input: i32) {
    // let mut conn = connect_db::connect().get_conn().unwrap();
    // let res = conn.exec_first("SELECT ID as id, FULL_NAME as full_name, PASSWORD as password, DATE_CREATED as date_created from PASSWORD_MANAGER where id = ?", (id_input, ));
    // // println!("{:?}", res);
    // let resp = assert_eq!(res.unwrap(), Some(res.ok()));
}

// pub fn get_oldpass() -> Vec<Password> {
// let mut conn = connect_db::connect().get_conn().unwrap();

// let res = conn
//     .query_map(
//         "select ID as id, FULL_NAME as full_name, PASSWORD as password, DATE_CREATED from PASSWORD_OLD",
//         |(id, full_name, password, date_created)| Password {
//             id,
//             full_name,
//             password,
//             date_created,
//         },
//     )
//     .unwrap();
// res
// }
