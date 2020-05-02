use rocket::delete;
use serde::Serialize;
use serde_json;
use crate::{
    file_id::FileId,
    password::Password,
    db::{TempfilesDatabaseConn, schemas::TempfilesDatabase},
    impl_responder
};
use super::ApiError;

#[derive(Serialize)]
pub struct ApiDeleteResponse {
    pub status: u16
}

impl_responder!(ApiDeleteResponse);


#[delete("/delete/<id>/<delete_password>")]
pub fn delete(
    id: FileId,
    delete_password: Password,
    db: TempfilesDatabaseConn
) -> Result<ApiDeleteResponse, ApiError> {

    match TempfilesDatabase::delete(&db, id.as_ref(), delete_password.as_ref())? {

        0 => Err(ApiError::not_found()),

        1 => Ok(ApiDeleteResponse {
            status: 200,
        }),

        _ => unreachable!()

    }

}
