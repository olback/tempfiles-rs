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
    pub size: i64,
    pub files: i64,
}

pub struct TempfilesDatabase;

impl TempfilesDatabase {
    pub async fn get_by_id(
        conn: &mut TempfilesDatabaseConn,
        id: String,
        keep_hours: u64,
    ) -> Result<Option<TempfilesDatabaseRow>, postgres::Error> {
        Ok(conn
            .run(move |c| {
                c.query(
                    // Make sure to only return files that have not expired and have not been viewed max_views times.
                    "select id, iv, content from tempfiles where id = $1 and (max_views is null or views < max_views) and (timestamp > now() - (interval '1 hours' * $2))",
                    &[&id, &(keep_hours as f64)],
                )
            })
            .await?
            .first()
            .map(|row| {
                Some(TempfilesDatabaseRow {
                    id: row.get("id"),
                    iv: row.get::<_, Vec<u8>>("iv").as_slice().try_into().unwrap(),
                    content: row.get("content"),
                    // views: row.get(3),
                    // max_views: row.get(4),
                    // delete_password: row.get(5),
                    // timestamp: row.get(6), // skip timestamp, get as f64 instead
                    // timestamp: row.get(7),
                })
            })
            .unwrap_or(None))
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
        }).await.map(|_| ())
    }

    pub async fn delete(
        conn: &mut TempfilesDatabaseConn,
        id: String,
        delete_password: String,
    ) -> Result<u64, postgres::Error> {
        conn.run(move |c| {
            c.execute(
                "DELETE FROM public.tempfiles WHERE id = $1 AND delete_password = $2",
                &[&id, &delete_password],
            )
        })
        .await
    }

    pub async fn increment_views(
        conn: &mut TempfilesDatabaseConn,
        id: String,
    ) -> Result<u64, postgres::Error> {
        conn.run(move |c| {
            c.execute(
                "UPDATE public.tempfiles SET views = views + 1 WHERE id = $1",
                &[&id],
            )
        })
        .await
    }

    pub async fn get_stats(
        conn: &mut TempfilesDatabaseConn,
    ) -> Result<TempfilesDatabaseStats, postgres::Error> {
        conn.run(move |c| c.query("select * from get_stats()", &[]))
            .await?
            .first()
            .map(|row| TempfilesDatabaseStats {
                size: (row.get::<_, Option<i64>>("total_size")).unwrap_or(0),
                files: (row.get::<_, Option<i64>>("amount")).unwrap_or(0),
            })
            .ok_or_else(|| unreachable!())
    }

    pub async fn cleanup(
        conn: &mut TempfilesDatabaseConn,
        keep_hours: u64,
    ) -> Result<u64, postgres::Error> {
        // Files which have max_views set are automatically deleted after they are viewed max_views times by the database.
        // See the 'max_views_trigger_func' function in the database schema for more information.
        conn.run(move |c| {
            c.execute(
                "delete from public.tempfiles WHERE (timestamp < now() - (interval '1 hours' * $1)) or (max_views is not null and views >= max_views)",
                &[&(keep_hours as f64)],
            )
        })
        .await
    }
}
