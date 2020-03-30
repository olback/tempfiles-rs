use rocket::{get};
use crate::utils::{FileId, Password};

#[get("/<id>/<password>")]
pub fn download(id: FileId, password: Password) -> Option<&'static str> {

    println!("id: {} password: {}", id, password);

    Some("ok")

}
