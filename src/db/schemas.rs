use rocket_contrib::databases::postgres;
use std::convert::TryInto;

#[derive(Debug)]
pub struct TempfilesDatabaseRow {
    pub id: String,
    pub iv: [u8; 12],
    pub content: Vec<u8>,
    // pub views: i32,
    // pub max_views: Option<i32>,
    // pub delete_password: String,
    // pub timestamp: f64
}

#[derive(Debug)]
pub struct TempfilesDatabaseStats {
    pub size: u32,
    pub files: u32
}

pub struct TempfilesDatabase;

impl TempfilesDatabase {

    pub fn get_by_id(conn: &postgres::Connection, id: &String) -> Result<Option<TempfilesDatabaseRow>, postgres::Error> {

        let rows = conn.query("select id, iv, content from tempfiles where id = $1", &[id])?;

        Ok(match rows.len() {

            1 => {

                let row = rows.get(0);

                Some(TempfilesDatabaseRow {
                    id: row.get(0),
                    iv: row.get::<_, Vec<u8>>(1).as_slice().try_into().unwrap(),
                    content: row.get(2),
                    // views: row.get(3),
                    // max_views: row.get(4),
                    // delete_password: row.get(5),
                    // timestamp: row.get(6), // skip timestamp, get as f64 instead
                    // timestamp: row.get(7),
                })

            },

            _ => None

        })

    }

    pub fn insert(
        conn: &postgres::Connection,
        id: &String,
        iv: &[u8; 12],
        content: &Vec<u8>,
        max_views: &Option<i32>,
        delete_password: &String
    ) -> Result<(), postgres::Error> {

        let iv_vec = iv.to_vec();

        conn.execute(
            "INSERT INTO tempfiles (id, iv, content, max_views, delete_password) VALUES ($1, $2, $3, $4, $5)",
            &[
                &id,
                &iv_vec,
                &content,
                &max_views,
                &delete_password
            ]
        )?;

        Ok(())

    }

    pub fn delete(conn: &postgres::Connection, id: &String, delete_password: &String) -> Result<u64, postgres::Error> {

        Ok(conn.execute("DELETE FROM tempfiles WHERE id = $1 AND delete_password = $2", &[&id, &delete_password])?)

    }

    pub fn increment_views(conn: &postgres::Connection, id: &String) -> Result<u64, postgres::Error> {

        Ok(conn.execute("UPDATE tempfiles SET views = views + 1 WHERE id = $1", &[&id])?)

    }

    pub fn get_stats(conn: &postgres::Connection) -> Result<TempfilesDatabaseStats, postgres::Error> {

        let rows = conn.query("select * from get_stats()", &[])?;

        let row = rows.get(0);

        Ok(TempfilesDatabaseStats {
            size: (row.get::<_, i64>(0)) as u32,
            files: (row.get::<_, i64>(1)) as u32,
        })

    }


}
