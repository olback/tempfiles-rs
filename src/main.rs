#![feature(proc_macro_hygiene, decl_macro)]

use rocket::Rocket;

mod routes;
mod utils;

fn main() {

    Rocket::ignite()
    .mount("/", rocket::routes![
        routes::site::index
    ])
    .mount("/d", rocket::routes![
        routes::download::download
    ])
    .mount("/api", rocket::routes![
        routes::api::upload,
        // routes::api::delete
    ])
    .launch();

}
