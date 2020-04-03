use rocket::{get};
use crate::{
    db::{
        TempfilesDatabaseConn,
        schemas::TempfilesDatabase
    },
    file_id::{
        FileId,
        Password
    }
};

#[get("/<id>/<password>")]
pub fn download(id: FileId, password: Password, db: TempfilesDatabaseConn) -> Option<&'static str> {

    println!("id: {} password: {}", id, password);

    let res = TempfilesDatabase::get_id(&db, id.into());
    println!("{:#?}", res);

    Some("ok")

}
