use rocket::get;
use serde::Serialize;
use serde_json;
use crate::{
    impl_responder,
    db::{
        TempfilesDatabaseConn,
        schemas::TempfilesDatabase
    }
};
use super::ApiError;

#[derive(Serialize)]
pub struct ApiStatsResponse {
    pub status: u16,
    pub size: u32,
    pub files: u32
}

impl_responder!(ApiStatsResponse);

#[get("/stats")]
pub fn stats(db: TempfilesDatabaseConn) -> Result<ApiStatsResponse, ApiError> {

    let res = TempfilesDatabase::get_stats(&db)?;

    Ok(ApiStatsResponse {
        status: 200,
        size: res.size,
        files: res.files
    })

}
