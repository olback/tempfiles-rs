use crate::{
    config::TempfilesConfig,
    content::Content,
    crypto::Crypto,
    db::{schemas::TempfilesDatabase, TempfilesDatabaseConn},
    file_id::FileId,
    password::Password,
    routes::api::ApiError,
};
use rocket::{get, State};

#[get("/<id>/<password>")]
pub async fn download(
    id: FileId,
    password: Password,
    tc: &State<TempfilesConfig>,
    mut db: TempfilesDatabaseConn,
) -> Result<Option<Content>, ApiError> {
    match TempfilesDatabase::get_by_id(&mut db, id.into(), tc.keep_hours).await? {
        Some(data) => {
            let content_bytes = match Crypto::decrypt(data.iv, password.as_array32(), &data.content)
            {
                Ok(v) => v,
                Err(_) => return Err(ApiError::not_found()),
            };

            let content = bincode::deserialize::<Content>(&content_bytes)?;

            if let Err(e) = TempfilesDatabase::increment_views(&mut db, data.id.clone()).await {
                rocket::error!("Failed to increment views: {e}");
            }

            Ok(Some(content))
        }
        None => Ok(None),
    }
}
