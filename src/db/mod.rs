use rocket_contrib::{
    database,
    databases::postgres
};

pub mod schemas;

#[database("tempfiles")]
pub struct TempfilesDatabaseConn(postgres::Connection);
