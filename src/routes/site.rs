use {
    crate::config::TempfilesConfig,
    rocket::{get, State},
    rocket_dyn_templates::Template,
    serde::Serialize,
};

#[derive(Serialize)]
struct IndexContext<'c> {
    version: &'c str,
    max_file_size: usize,
    name: String,
    url: String,
}

impl<'c> IndexContext<'c> {
    fn new(tc: &TempfilesConfig) -> Self {
        Self {
            version: include_str!("../../version.txt").trim(),
            max_file_size: tc.max_file_size / (1024 * 1024),
            name: tc.name.clone(),
            url: tc.base_url.clone(),
        }
    }
}

#[get("/")]
pub fn index(tc: &State<TempfilesConfig>) -> Template {
    Template::render("index", IndexContext::new(tc))
}

#[get("/<tab>")]
pub fn index_tab(tab: &str, tc: &State<TempfilesConfig>) -> Option<Template> {
    match tab {
        "upload" => Some(Template::render("index", IndexContext::new(tc))),
        "download" => Some(Template::render("index", IndexContext::new(tc))),
        "delete" => Some(Template::render("index", IndexContext::new(tc))),
        "sharex" => Some(Template::render("index", IndexContext::new(tc))),
        "api" => Some(Template::render("index", IndexContext::new(tc))),
        _ => None,
    }
}
