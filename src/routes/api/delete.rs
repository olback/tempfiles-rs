use super::ApiError;
use crate::{
    db::{schemas::TempfilesDatabase, TempfilesDatabaseConn},
    file_id::FileId,
    impl_responder,
    password::Password,
};
use rocket::delete;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct ApiDeleteResponse {
    pub status: u16,
}

impl_responder!(ApiDeleteResponse);

#[delete("/delete/<id>/<delete_password>")]
pub async fn delete(
    id: FileId,
    delete_password: Password,
    mut db: TempfilesDatabaseConn,
) -> Result<ApiDeleteResponse, ApiError> {
    match TempfilesDatabase::delete(&mut db, id.into(), delete_password.into()).await? {
        0 => Err(ApiError::not_found()),
        1 => Ok(ApiDeleteResponse { status: 200 }),
        _ => unreachable!(),
    }
}
