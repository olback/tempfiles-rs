use serde::Deserialize;

#[derive(Deserialize)]
pub struct TempfilesConfig {
    pub max_file_size: usize
}

impl TempfilesConfig {

    pub fn from_rocket(rocket_config: &rocket::config::Config) -> Self {

        Self {
            max_file_size: rocket_config.limits.get("file").unwrap_or(rocket_config.limits.get("forms").unwrap()) as usize
        }

    }

}
