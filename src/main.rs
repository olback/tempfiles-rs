#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{
    routes,
    catchers,
    Rocket,
    fairing::AdHoc
};
use rocket_contrib::{
    templates::Template,
    serve::StaticFiles
};

mod routes;
mod catchers;
mod macros;
mod file_id;
mod config;
mod db;

fn main() {

    Rocket::ignite()
    .mount("/", routes![
        routes::site::index,
        routes::site::index_tab,
    ])
    .mount("/assets", StaticFiles::from("assets"))
    .mount("/d", routes![
        routes::download::download
    ])
    .mount("/api", routes![
        // routes::api::get,
        routes::api::upload::upload,
        // routes::api::delete
    ])
    .register(catchers![
        catchers::client::bad_request,
        catchers::client::forbidden,
        catchers::client::not_found,
        catchers::client::unprocessable_entity,
        catchers::server::internal_server_error
    ])
    .attach(db::TempfilesDatabaseConn::fairing())
    .attach(AdHoc::on_attach("Tempfiles config", |rocket| {
        let tempfiles_config = config::TempfilesConfig::from_rocket(rocket.config());
        Ok(rocket.manage(tempfiles_config))
    }))
    .attach(Template::fairing())
    .launch();

}
