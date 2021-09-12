use rocket_sync_db_pools::{database, postgres};

pub mod schemas;

#[database("tempfiles")]
pub struct TempfilesDatabaseConn(postgres::Client);
