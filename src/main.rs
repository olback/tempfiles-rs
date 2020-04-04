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
mod password;
mod config;
mod db;
mod content;
mod crypto;

fn main() {

    // Wait for database to start
    std::thread::sleep(std::time::Duration::from_secs(1));

    Rocket::ignite()
    .mount("/", routes![
        routes::site::index,
        routes::site::index_tab,
        routes::sharex::sharex
    ])
    .mount("/assets", StaticFiles::from("assets"))
    .mount("/d", routes![
        routes::download::download
    ])
    .mount("/api", routes![
        routes::api::upload::upload,
        routes::api::delete::delete
    ])
    .register(catchers![
        catchers::client::bad_request,
        catchers::client::forbidden,
        catchers::client::not_found,
        catchers::client::unprocessable_entity,
        catchers::server::internal_server_error,
        catchers::server::service_unavailable
    ])
    .attach(db::TempfilesDatabaseConn::fairing())
    .attach(AdHoc::on_attach("Tempfiles config", |rocket| {
        let tempfiles_config = config::TempfilesConfig::from_rocket(rocket.config());
        Ok(rocket.manage(tempfiles_config))
    }))
    .attach(Template::fairing())
    .launch();

}
