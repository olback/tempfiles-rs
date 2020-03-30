use rocket::get;

#[get("/")]
pub fn index() -> &'static str {

    "hello"

}
