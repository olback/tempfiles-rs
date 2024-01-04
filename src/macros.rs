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
macro_rules! impl_responder {
    ($t:ty) => {
        impl<'r> rocket::response::Responder<'r, 'static> for $t {
            fn respond_to(self, _: &rocket::request::Request) -> rocket::response::Result<'static> {
                let content = serde_json::to_string_pretty(&self).unwrap();
                rocket::response::Response::build()
                    .header(rocket::http::ContentType::JSON)
                    .status(rocket::http::Status::from_code(self.status).unwrap())
                    .sized_body(content.len(), std::io::Cursor::new(content))
                    .ok()
            }
        }
    };
}
