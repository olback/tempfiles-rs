use rocket::{
    http::{ContentType, Header, Status},
    request::Request,
    response::{self, Responder, Response},
};
use serde::{Deserialize, Serialize};
use std::{io::Cursor, str::FromStr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub data: Vec<u8>,
}

impl<'r> Responder<'r, 'static> for Content {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
        let default_content_type = ContentType::from_str("application/octet-stream").unwrap();

        let content_type = match self.content_type {
            Some(ct) => ContentType::from_str(&ct).unwrap_or(default_content_type),
            None => default_content_type,
        };

        let content_disposition = match self.filename {
            Some(name) => format!("inline; filename={}", name),
            None => "inline".into(),
        };

        Response::build()
            .status(Status::Ok)
            .header(content_type)
            .header(Header::new("content-disposition", content_disposition))
            .streamed_body(Cursor::new(self.data))
            .ok()
    }
}
