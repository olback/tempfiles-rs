pub struct TempfilesConfig {
    pub max_file_size: usize,
    pub base_url: String,
    pub name: String,
}

impl TempfilesConfig {
    pub fn from_env() -> Self {
        Self {
            max_file_size: std::env::var("ROCKET_LIMITS_BYTES")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            base_url: std::env::var("TEMPFILES_BASE_URL").unwrap(),
            name: std::env::var("TEMPFILES_NAME").unwrap(),
        }
    }
}
