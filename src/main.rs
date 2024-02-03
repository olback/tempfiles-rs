use {
    rocket::{catchers, fairing::AdHoc, fs::FileServer, routes, Rocket},
    rocket_dyn_templates::Template,
};

mod catchers;
mod cleanup;
mod config;
mod content;
mod crypto;
mod db;
mod file_id;
mod macros;
mod password;
mod routes;

#[rocket::launch]
async fn launch() -> _ {
    Rocket::build()
        .mount(
            "/",
            routes![
                routes::site::index,
                routes::site::index_tab,
                routes::sharex::sharex
            ],
        )
        .mount("/assets", FileServer::from("assets"))
        .mount("/d", routes![routes::download::download])
        .mount(
            "/api",
            routes![
                routes::api::upload::upload,
                routes::api::delete::delete,
                routes::api::metadata::metadata,
                routes::api::stats::stats
            ],
        )
        .register(
            "/",
            catchers![
                catchers::client::bad_request,
                catchers::client::forbidden,
                catchers::client::not_found,
                catchers::client::unprocessable_entity,
                catchers::server::internal_server_error,
                catchers::server::service_unavailable
            ],
        )
        .attach(cleanup::Cleanup::fairing())
        .attach(Template::fairing())
        .attach(db::TempfilesDatabaseConn::fairing())
        .attach(AdHoc::on_ignite("Tempfiles config", |rocket| async {
            let tempfiles_config = config::TempfilesConfig::from_env();
            rocket.manage(tempfiles_config)
        }))
}
