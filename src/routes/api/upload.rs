use rocket::{
    post,
    Data,
    State,
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
use std::io::Cursor;
use crate::{
    file_id::{FileId, Password},
    config::TempfilesConfig,
    db::TempfilesDatabaseConn
};
use super::ApiError;
use std::io::Read;
use aead::{Aead, NewAead, generic_array::GenericArray};
use aes_gcm_siv;
use rand::{self, Rng};
use diesel::prelude::*;

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
            .status(Status::from_code(self.status).unwrap())
            .sized_body(Cursor::new(serde_json::to_string_pretty(&self).unwrap()))
            .ok()

    }

}

#[post("/upload?<filename>&<maxviews>", data = "<data>")]
pub fn upload(
    filename: Option<&RawStr>,
    maxviews: Option<usize>,
    content_type: Option<&ContentType>,
    data: Data,
    tc: State<TempfilesConfig>,
    db: TempfilesDatabaseConn
) -> Result<ApiUploadResponse, ApiError> {

    let mut raw_data = Vec::<u8>::new();
    let size = data.open().take(tc.max_file_size as u64).read_to_end(&mut raw_data)?;

    // println!("size: {}", size);
    // println!("{:?}", filename);
    // println!("{:?}", maxviews);
    // println!("{:?}", content_type);
    // println!("vec: {:?}", raw_data);

    if size == 0 {
        return Err(ApiError::new("File may not be empty", 422))
    } else if size == tc.max_file_size {
        return Err(ApiError::new("File too large", 422))
    }

    let id = FileId::new(16);
    let password = Password::new(32); // This has to be 32 bytes!
    let delete_password = Password::new(16);
    let key = GenericArray::from(password.as_array32());
    let iv = GenericArray::from(rand::thread_rng().gen::<[u8; 12]>());

    // let cipher = aes_gcm_siv::Aes256GcmSiv::new(key);
    // let enc = cipher.encrypt(&iv, &*raw_data)?;

    Ok(ApiUploadResponse {
        status: 201,
        id: id.into(),
        password: password.into(),
        delete_password: delete_password.into(),
    })

}
