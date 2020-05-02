use rocket::{
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
use std::io::Cursor;
use crate::internal_server_error_from;

pub mod upload;
pub mod delete;
pub mod metadata;

#[derive(Serialize, Debug)]
pub struct ApiError {
    status: u16,
    message: String
}

impl ApiError {

    pub fn new<M>(message: M, status: u16) -> Self
        where M: std::fmt::Display {

        Self {
            status: status,
            message: format!("{}", message)
        }

    }

    pub fn not_found() -> Self {

        Self::new("Not Found", 404)

    }

}

impl<'a> Responder<'a> for ApiError {

    fn respond_to(self, _: &Request) -> response::Result<'a> {

        Response::build()
            .header(ContentType::JSON)
            .status(Status::from_code(self.status).unwrap())
            .sized_body(Cursor::new(serde_json::to_string_pretty(&self).unwrap()))
            .ok()

    }

}

internal_server_error_from!(std::io::Error);
internal_server_error_from!(aead::Error);
internal_server_error_from!(Box<bincode::ErrorKind>);
internal_server_error_from!(rocket_contrib::databases::postgres::error::Error);
