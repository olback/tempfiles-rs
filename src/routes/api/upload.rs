use rocket::{
    post,
    Data,
    http::{
        ContentType,
        RawStr,
        Status
    },
    request::Request,
    response::{
        self,
        Response,
        Responder
    }
};
use serde::Serialize;
use serde_json;
use std::{io::Cursor, env};
use crate::utils::{FileId, Password};
use super::ApiError;
use std::io::Read;

#[derive(Serialize)]
pub struct ApiUploadResponse {
    pub status: u16,
    pub id: String,
    pub password: String,
    pub delete_password: String
}

impl<'a> Responder<'a> for ApiUploadResponse {

    fn respond_to(self, _: &Request) -> response::Result<'a> {

        Response::build()
            .header(ContentType::JSON)
            .status(Status::Ok)
            .sized_body(Cursor::new(serde_json::to_string_pretty(&self).unwrap()))
            .ok()

    }

}

#[post("/upload?<filename>&<maxviews>", data = "<data>")]
pub fn upload(filename: Option<&RawStr>, maxviews: Option<usize>, content_type: Option<&ContentType>, data: Data) -> Result<ApiUploadResponse, ApiError> {

    // Max upload size
    const max: usize = 26214400;

    let mut v = Vec::<u8>::new();
    let size = data.open().take(max as u64).read_to_end(&mut v)?;
    println!("size: {}", size);

    if size == max {

        return Err(ApiError::new("File too large", 422))

    }


    // drop(v);
    // data.stream_to_file(env::temp_dir().join("upload.txt"))
    //     .map(|n| n.to_string());

    Ok(ApiUploadResponse {
        status: 200,
        id: "aaa".into(),
        password: "bbb".into(),
        delete_password: "ccc".into(),
    })

}
