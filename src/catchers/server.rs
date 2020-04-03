use rocket::{catch, Request};
use rocket_contrib::templates::Template;
use super::ErrorContext;

#[catch(500)]
pub fn internal_server_error(_: &Request) -> Template {

    let context = ErrorContext {
        status: 500,
        message: "Internal Server Error".into()
    };

    Template::render("error", &context)

}
