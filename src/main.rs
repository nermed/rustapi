// add our routes and services modules
mod dbase;
mod routes;
mod services;
mod models;

// import our routes
use routes::psswrd::get_allpass;
use routes::psswrd::get_oldpass;
use routes::psswrd::get_onepass;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![get_allpass, get_oldpass, get_onepass])
}
