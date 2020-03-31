use rocket::{
    post,
    delete,
    response::{status},
    Data,
    http::{ContentType, RawStr, Status}
};
use serde::Serialize;
use rocket_contrib::json::Json;
use std::{io, env};
use crate::utils::{FileId, Password};

#[derive(Serialize)]
pub struct ApiError {
    status: u16,
    message: String
}

#[derive(Serialize)]
pub struct ApiUploadResponse {
    pub status: u16,
    pub id: String,
    pub password: String,
    pub delete_password: String
}

#[post("/upload?<filename>&<maxviews>", format = "any", data = "<data>")]
pub fn upload(filename: Option<&RawStr>, maxviews: Option<usize>, content_type: &ContentType, data: Data) -> Result<Json<ApiUploadResponse>, ()> {

    let file_id = FileId::new(16);
    let password = Password::new(16);
    let delete_password = Password::new(16);

    println!("{:#?} {:#?}", filename, maxviews);
    println!("{:#?}", content_type);

    data.stream_to_file(env::temp_dir().join("upload.txt"))
        .map(|n| n.to_string());

    Ok(Json(ApiUploadResponse {
        status: 200,
        id: file_id.to_string(),
        password: password.to_string(),
        delete_password: delete_password.to_string()
    }))

}

// #[delete("/delete", format = "json", data = "<data>")]
// pub fn delete(data: DeleteFile) -> Result<(), ()> {

//     unimplemented!()

// }

