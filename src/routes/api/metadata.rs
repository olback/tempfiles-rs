use super::ApiError;
use crate::{
    content::Content,
    crypto::Crypto,
    db::{schemas::TempfilesDatabase, TempfilesDatabaseConn},
    file_id::FileId,
    impl_responder,
    password::Password,
};
use rocket::get;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct ApiMetadataResponse {
    pub status: u16,
    pub content_type: Option<String>,
    pub content_length: usize,
    pub filename: Option<String>,
}

impl_responder!(ApiMetadataResponse);

#[get("/metadata/<id>/<password>")]
pub async fn metadata(
    id: FileId,
    password: Password,
    mut db: TempfilesDatabaseConn,
) -> Result<ApiMetadataResponse, ApiError> {
    let row = TempfilesDatabase::get_by_id(&mut db, id.into()).await?;

    if let Some(ref data) = row {
        let ref content_bytes = match Crypto::decrypt(data.iv, password.as_array32(), &data.content)
        {
            Ok(v) => v,
            Err(_) => return Err(ApiError::not_found()),
        };

        let content = bincode::deserialize::<Content>(content_bytes)?;

        return Ok(ApiMetadataResponse {
            status: 200,
            content_type: content.content_type,
            content_length: content.data.len(),
            filename: content.filename,
        });
    }

    Err(ApiError::not_found())
}
