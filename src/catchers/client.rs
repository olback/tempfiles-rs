use {
    super::ErrorContext,
    rocket::{catch, Request},
    rocket_dyn_templates::Template,
};

#[catch(400)]
pub fn bad_request(_: &Request) -> Template {
    let context = ErrorContext {
        status: 400,
        message: "Bad request".into(),
    };

    Template::render("error", context)
}

#[catch(403)]
pub fn forbidden(_: &Request) -> Template {
    let context = ErrorContext {
        status: 403,
        message: "Forbidden".into(),
    };

    Template::render("error", context)
}

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let context = ErrorContext {
        status: 404,
        message: format!("Could not find {}", req.uri()),
    };

    Template::render("error", context)
}

#[catch(422)]
pub fn unprocessable_entity(_: &Request) -> Template {
    let context = ErrorContext {
        status: 422,
        message: "Unprocessable entity".into(),
    };

    Template::render("error", context)
}
