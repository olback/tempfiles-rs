pub struct TempfilesConfig {
    pub max_file_size: usize,
    pub base_url: String,
    pub name: String
}

impl TempfilesConfig {

    pub fn from_rocket(rocket_config: &rocket::config::Config) -> Self {

        Self {
            max_file_size: rocket_config.limits.get("files").unwrap() as usize,
            base_url: rocket_config.get_str("base_url").unwrap().to_string(),
            name: rocket_config.get_str("name").unwrap().to_string()
        }

    }

}
