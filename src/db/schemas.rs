use rocket_contrib::databases::postgres;

#[derive(Debug)]
pub struct TempfilesDatabase {
    id: String,
    iv: [u8; 12],
    content: Vec<u8>,
    views: usize,
    max_views: usize,
    timestamp: usize
}

impl TempfilesDatabase {

    pub fn get_id(conn: &postgres::Connection, id: String) -> Result<Option<Self>, postgres::Error> {


        let rows = conn.query("SELECT * FROM tempfiles WHERE ID=$1", &[&id])?;
        println!("{:#?}", rows);


        Ok(None)

    }

    pub fn insert(&self, conn: &postgres::Connection) -> Result<(), postgres::Error> {

        unimplemented!()

    }

}
