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
    password::Password
};

#[get("/<id>/<password>")]
pub fn download(id: FileId, password: Password, db: TempfilesDatabaseConn) -> Result<Option<Content>, ApiError> {

    let row = TempfilesDatabase::get_by_id(&db, id.as_ref())?;

    Ok(match row {

        Some(ref data) => {

            match Crypto::decrypt(data.iv, password.as_array32(), &data.content) {

                Ok(ref content_bytes) => {

                    match bincode::deserialize::<Content>(content_bytes) {

                        Ok(content) => {

                            drop(TempfilesDatabase::increment_views(&db, &data.id));

                            // match data.max_views {

                            //     drop(TempfilesDatabase::increment_views(&db, &data.id))

                            //     Some(max_views) => {

                            //         if data.views + 1 == max_views {

                            //             drop(TempfilesDatabase::delete(&db, &data.id, &data.delete_password));

                            //         } else {

                            //             drop(TempfilesDatabase::increment_views(&db, &data.id))

                            //         }

                            //     },
                            //     None => {} // No reason to count views if max_views is None

                            // }

                            Some(content)
                        },
                        Err(_) => None

                    }

                },

                Err(_) => None

            }

        },

        None => None

    })

}
