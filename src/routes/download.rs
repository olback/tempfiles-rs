use crate::{
    content::Content,
    crypto::Crypto,
    db::{schemas::TempfilesDatabase, TempfilesDatabaseConn},
    file_id::FileId,
    password::Password,
    routes::api::ApiError,
    some,
};
use rocket::get;

#[get("/<id>/<password>")]
pub async fn download(
    id: FileId,
    password: Password,
    mut db: TempfilesDatabaseConn,
) -> Result<Option<Content>, ApiError> {
    let row = TempfilesDatabase::get_by_id(&mut db, id.into()).await?;

    if let Some(ref data) = row {
        let ref content_bytes = some!(Crypto::decrypt(
            data.iv,
            password.as_array32(),
            &data.content
        ));
        let content = bincode::deserialize::<Content>(content_bytes)?;

        drop(TempfilesDatabase::increment_views(&mut db, data.id.clone()));

        return Ok(Some(content));
    }

    Ok(None)
}
