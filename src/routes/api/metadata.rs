use rocket::get;
use serde::Serialize;
use serde_json;
use crate::{
    file_id::FileId,
    password::Password,
    db::{TempfilesDatabaseConn, schemas::TempfilesDatabase},
    content::Content,
    crypto::Crypto,
    impl_responder
};
use super::ApiError;

#[derive(Serialize)]
pub struct ApiMetadataResponse {
    pub status: u16,
    pub content_type: Option<String>,
    pub content_length: usize,
    pub filename: Option<String>
}

impl_responder!(ApiMetadataResponse);

#[get("/metadata/<id>/<password>")]
pub fn metadata(
    id: FileId,
    password: Password,
    db: TempfilesDatabaseConn
) -> Result<ApiMetadataResponse, ApiError> {

    let row = TempfilesDatabase::get_by_id(&db, id.as_ref())?;

    if let Some(ref data) = row {

        let ref content_bytes = match Crypto::decrypt(data.iv, password.as_array32(), &data.content) {
            Ok(v) => v,
            Err(_) => return Err(ApiError::not_found())
        };

        let content = bincode::deserialize::<Content>(content_bytes)?;

        return Ok(ApiMetadataResponse {
            status: 200,
            content_type: content.content_type,
            content_length: content.data.len(),
            filename: content.filename
        })

    }

    Err(ApiError::not_found())

}
