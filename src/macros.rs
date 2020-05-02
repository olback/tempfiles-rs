#[macro_export]
macro_rules! internal_server_error_from {
    ($t:ty) => {
        impl From<$t> for ApiError {
            fn from(err: $t) -> ApiError {
                eprintln!("{:#?}", err);
                $crate::routes::api::ApiError::new("Internal Server Error", 500)
            }
        }
    };
}

#[macro_export]
macro_rules! some {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(_) => return Ok(None)
        }
    };
}

#[macro_export]
macro_rules! impl_responder {
    ($t:ty) => {
        impl<'a> rocket::response::Responder<'a> for $t {
            fn respond_to(self, _: &rocket::request::Request) -> rocket::response::Result<'a> {
                rocket::response::Response::build()
                    .header(rocket::http::ContentType::JSON)
                    .status(rocket::http::Status::from_code(self.status).unwrap())
                    .sized_body(std::io::Cursor::new(serde_json::to_string_pretty(&self).unwrap()))
                    .ok()
            }
        }
    };
}
