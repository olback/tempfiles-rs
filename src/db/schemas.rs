use super::TempfilesDatabaseConn;

use {rocket_sync_db_pools::postgres, std::convert::TryInto};

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
    pub files: u32,
}

pub struct TempfilesDatabase;

impl TempfilesDatabase {
    pub async fn get_by_id(
        conn: &mut TempfilesDatabaseConn,
        id: String,
    ) -> Result<Option<TempfilesDatabaseRow>, postgres::Error> {
        let rows = conn
            .run(move |c| {
                c.query(
                    "select id, iv, content from tempfiles where id = $1",
                    &[&id],
                )
            })
            .await?;
        // let rows = conn.query("select id, iv, content from tempfiles where id = $1", &[id])?;

        Ok(match rows.len() {
            1 => {
                let row = rows.get(0).unwrap();

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
            }

            _ => None,
        })
    }

    pub async fn insert(
        conn: &mut TempfilesDatabaseConn,
        id: String,
        iv: &[u8; 12],
        content: Vec<u8>,
        max_views: Option<i32>,
        delete_password: String,
    ) -> Result<(), postgres::Error> {
        let iv_vec = iv.to_vec();

        conn.run(move |c| {
            c.execute(
                "INSERT INTO tempfiles (id, iv, content, max_views, delete_password) VALUES ($1, $2, $3, $4, $5)",
                &[
                    &id,
                    &iv_vec,
                    &content,
                    &max_views,
                    &delete_password
                ]
            )
        }).await?;

        Ok(())
    }

    pub async fn delete(
        conn: &mut TempfilesDatabaseConn,
        id: String,
        delete_password: String,
    ) -> Result<u64, postgres::Error> {
        Ok(conn
            .run(move |c| {
                c.execute(
                    "DELETE FROM tempfiles WHERE id = $1 AND delete_password = $2",
                    &[&id, &delete_password],
                )
            })
            .await?)
    }

    pub async fn increment_views(
        conn: &mut TempfilesDatabaseConn,
        id: String,
    ) -> Result<u64, postgres::Error> {
        Ok(conn
            .run(move |c| {
                c.execute(
                    "UPDATE tempfiles SET views = views + 1 WHERE id = $1",
                    &[&id],
                )
            })
            .await?)
    }

    pub async fn get_stats(
        conn: &mut TempfilesDatabaseConn,
    ) -> Result<TempfilesDatabaseStats, postgres::Error> {
        let rows = conn
            .run(move |c| c.query("select * from get_stats()", &[]))
            .await?;

        let row = rows.get(0).unwrap();

        Ok(TempfilesDatabaseStats {
            size: (row.get::<_, Option<i64>>(0)).unwrap_or(0) as u32,
            files: (row.get::<_, Option<i64>>(1)).unwrap_or(0) as u32,
        })
    }
}
