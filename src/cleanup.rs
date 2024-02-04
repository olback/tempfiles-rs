use crate::{
    config::TempfilesConfig,
    db::{schemas::TempfilesDatabase, TempfilesDatabaseConn},
};
use rocket::{
    async_trait, fairing, fairing::Kind as FairingKind, http::Status, Build, Orbit, Rocket, State,
};

#[derive(Clone)]
pub struct Cleanup;

impl Cleanup {
    pub fn fairing() -> impl fairing::Fairing {
        Self
    }
}

#[async_trait]
impl fairing::Fairing for Cleanup {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Cleanup",
            kind: FairingKind::Ignite | FairingKind::Liftoff | FairingKind::Shutdown,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let r = rocket.mount("/", rocket::routes![endpoint]);
        fairing::Result::Ok(r)
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        let config = rocket.state::<TempfilesConfig>().unwrap();
        let base_url = config.base_url.clone();
        let key = config.cleanup_key.clone();
        let cleanup_url = format!("{base_url}/cleanup?key={key}");
        let cleanup_interval = config.cleanup_interval;

        if cleanup_interval > 0 {
            tokio::task::spawn(async move {
                let mut interval =
                    tokio::time::interval(tokio::time::Duration::from_secs(cleanup_interval));
                loop {
                    interval.tick().await;
                    match reqwest::get(&cleanup_url).await {
                        Ok(res) => {
                            if res.status().is_success() {
                                match res.text().await {
                                    Ok(text) => {
                                        rocket::info!("Cleanup: success: {text}");
                                    }
                                    Err(e) => {
                                        rocket::error!("Cleanup: success: {e}");
                                    }
                                }
                            } else {
                                let status = res.status();
                                match res.text().await {
                                    Ok(text) => {
                                        rocket::info!("Cleanup: error: {status}: {text}");
                                    }
                                    Err(e) => {
                                        rocket::error!("Cleanup: error: {status}: {e}");
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            rocket::error!("Cleanup: error: {}", e);
                        }
                    }
                }
            });
        } else {
            rocket::warn!("Automatic cleanup disabled");
        }
    }
}

#[rocket::get("/cleanup?<key>", rank = 1)]
async fn endpoint(
    mut db: TempfilesDatabaseConn,
    config: &State<TempfilesConfig>,
    key: &str,
) -> (Status, String) {
    if config.cleanup_key != key {
        return (Status::Unauthorized, "invalid key".into());
    }

    match TempfilesDatabase::cleanup(&mut db, config.keep_hours).await {
        Ok(n_rows) => (
            Status::Ok,
            format!(
                "deleted {n_rows} files older than {} hours or with views >= max_views",
                config.keep_hours
            ),
        ),
        Err(e) => (Status::InternalServerError, format!("error: {}", e)),
    }
}
