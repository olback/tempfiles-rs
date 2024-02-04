use super::ApiError;
use crate::{
    db::{schemas::TempfilesDatabase, TempfilesDatabaseConn},
    impl_responder,
};
use rocket::get;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
pub struct ApiStatsResponse {
    pub status: u16,
    pub size: i64,
    pub files: i64,
}

impl_responder!(ApiStatsResponse);

#[get("/stats")]
pub async fn stats(mut db: TempfilesDatabaseConn) -> Result<ApiStatsResponse, ApiError> {
    let res = TempfilesDatabase::get_stats(&mut db).await?;

    Ok(ApiStatsResponse {
        status: 200,
        size: res.size,
        files: res.files,
    })
}
