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
    crypto::Crypto,
    content::Content,
    file_id::FileId,
    password::Password,
    config::TempfilesConfig,
    db::{TempfilesDatabaseConn, schemas::TempfilesDatabase}
};
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
            .status(Status::from_code(self.status).unwrap())
            .sized_body(Cursor::new(serde_json::to_string_pretty(&self).unwrap()))
            .ok()

    }

}

#[post("/upload?<filename>&<maxviews>", data = "<data>")]
pub fn upload(
    filename: Option<&RawStr>,
    maxviews: Option<i32>,
    content_type: Option<&ContentType>,
    data: Data,
    tc: State<TempfilesConfig>,
    db: TempfilesDatabaseConn
) -> Result<ApiUploadResponse, ApiError> {

    let mut content = Content {
        filename: filename.map(|f| f.to_string()),
        content_type: content_type.map(|ct| ct.to_string()),
        data: Vec::<u8>::with_capacity(3 * 1024 * 1024)
    };

    let size = data.open().take(tc.max_file_size as u64).read_to_end(&mut content.data)?;

    // Check for invalid data
    if size == 0 {
        return Err(ApiError::new("File may not be empty", 422))
    } else if size == tc.max_file_size {
        return Err(ApiError::new("File too large", 422))
    } else if maxviews.is_some() && Some(1) > maxviews {
        return Err(ApiError::new("Max views may not be below 1", 422))
    }

    let id = FileId::new(16);
    let password = Password::new();
    let delete_password = Password::new();

    let iv = Crypto::gen_iv();
    let content_bytes = bincode::serialize(&content)?;

    let enc = Crypto::encrypt(iv, password.as_array32(), &content_bytes)?;

    TempfilesDatabase::insert(&db, id.as_ref(), &iv, &enc, &maxviews, delete_password.as_ref())?;

    Ok(ApiUploadResponse {
        status: 201,
        id: id.into(),
        password: password.into(),
        delete_password: delete_password.into(),
    })

}
