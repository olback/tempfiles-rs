#[macro_export]
macro_rules! internal_server_error_from {
    ($t:ty) => {
        impl From<$t> for ApiError {
            fn from(err: $t) -> ApiError {
                // crate::routes::api::ApiError::new(err, 500)
                eprintln!("{:#?}", err);
                $crate::routes::api::ApiError::new("Internal Server Error", 500)
            }
        }
    };
}