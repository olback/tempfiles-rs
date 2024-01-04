use crate::{impl_responder, internal_server_error_from};
use serde::Serialize;

pub mod delete;
pub mod metadata;
pub mod stats;
pub mod upload;

#[derive(Serialize, Debug)]
pub struct ApiError {
    status: u16,
    message: String,
}

impl ApiError {
    pub fn new<M>(message: M, status: u16) -> Self
    where
        M: std::fmt::Display,
    {
        Self {
            status,
            message: format!("{}", message),
        }
    }

    pub fn not_found() -> Self {
        Self::new("Not Found", 404)
    }
}

impl_responder!(ApiError);

internal_server_error_from!(std::io::Error);
internal_server_error_from!(aead::Error);
internal_server_error_from!(Box<bincode::ErrorKind>);
internal_server_error_from!(rocket_sync_db_pools::postgres::Error);
