use rocket::{get, http::RawStr};
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct IndexContext<'c> {
    version: &'c str
}

impl<'c> IndexContext<'c> {

    fn new() -> Self {
        Self {
            version: std::env!("CARGO_PKG_VERSION")
        }
    }

}

#[get("/")]
pub fn index() -> Template {

    Template::render("index", IndexContext::new())

}

#[get("/<tab>")]
pub fn index_tab(tab: String) -> Option<Template> {

    match tab.as_str() {
        "upload" => Some(Template::render("index", IndexContext::new())),
        "download" => Some(Template::render("index", IndexContext::new())),
        "delete" => Some(Template::render("index", IndexContext::new())),
        "sharex" => Some(Template::render("index", IndexContext::new())),
        "api" => Some(Template::render("index", IndexContext::new())),
        _ => None
    }

}
