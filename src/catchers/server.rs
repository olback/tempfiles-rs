use {
    super::ErrorContext,
    rocket::{catch, Request},
    rocket_dyn_templates::Template,
};

#[catch(500)]
pub fn internal_server_error(_: &Request) -> Template {
    let context = ErrorContext {
        status: 500,
        message: "Internal Server Error".into(),
    };

    Template::render("error", &context)
}

#[catch(503)]
pub fn service_unavailable(_: &Request) -> Template {
    let context = ErrorContext {
        status: 503,
        message: "Service Unavailable".into(),
    };

    Template::render("error", &context)
}
