use rocket::{
    delete,
    http::{
        ContentType,
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
    file_id::FileId,
    password::Password,
    db::{TempfilesDatabaseConn, schemas::TempfilesDatabase}
};
use super::ApiError;

#[derive(Serialize)]
pub struct ApiDeleteResponse {
    pub status: u16
}

impl<'a> Responder<'a> for ApiDeleteResponse {

    fn respond_to(self, _: &Request) -> response::Result<'a> {

        Response::build()
            .header(ContentType::JSON)
            .status(Status::from_code(self.status).unwrap())
            .sized_body(Cursor::new(serde_json::to_string_pretty(&self).unwrap()))
            .ok()

    }

}

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
