pub mod client;
pub mod server;

use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorContext {
    status: u16,
    message: String
}
