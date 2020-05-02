use rocket::get;
use crate::{
    crypto::Crypto,
    routes::api::ApiError,
    content::Content,
    db::{
        TempfilesDatabaseConn,
        schemas::TempfilesDatabase
    },
    file_id::FileId,
    password::Password,
    some
};

#[get("/<id>/<password>")]
pub fn download(id: FileId, password: Password, db: TempfilesDatabaseConn) -> Result<Option<Content>, ApiError> {

    let row = TempfilesDatabase::get_by_id(&db, id.as_ref())?;

    if let Some(ref data) = row {

        let ref content_bytes = some!(Crypto::decrypt(data.iv, password.as_array32(), &data.content));
        let content = bincode::deserialize::<Content>(content_bytes)?;

        drop(TempfilesDatabase::increment_views(&db, &data.id));

        return Ok(Some(content))

    }

    Ok(None)

}
