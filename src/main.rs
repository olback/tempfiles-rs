#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{self, Rocket};
use rocket_contrib::{
    templates::Template,
    serve::StaticFiles
};

mod routes;
mod utils;
mod macros;

fn main() {

    Rocket::ignite()
    .mount("/", rocket::routes![
        routes::site::index,
        routes::site::index_tab,
    ])
    .mount("/assets", StaticFiles::from("assets"))
    .mount("/d", rocket::routes![
        routes::download::download
    ])
    .mount("/api", rocket::routes![
        // routes::api::get,
        routes::api::upload::upload,
        // routes::api::delete
    ])
    .attach(Template::fairing())
    .launch();

}
