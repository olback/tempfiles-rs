pub struct TempfilesConfig {
    pub max_file_size: usize,
    pub max_views: i32,
    pub base_url: String,
    pub name: String,
    pub keep_hours: u64,       // hours to keep files
    pub cleanup_interval: u64, // seconds, 0 to disable automatic cleanup
    pub cleanup_key: String,   // secret key for cleanup
}

impl TempfilesConfig {
    pub fn from_env() -> Self {
        Self {
            max_file_size: std::env::var("ROCKET_LIMITS_BYTES")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            max_views: std::env::var("TEMPFILES_MAX_VIEWS")
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            base_url: std::env::var("TEMPFILES_BASE_URL").unwrap(),
            name: std::env::var("TEMPFILES_NAME").unwrap(),
            keep_hours: std::env::var("TEMPFILES_KEEP_HOURS")
                .map(|s| s.parse().unwrap())
                .unwrap_or_else(|_| {
                    rocket::warn!("TEMPFILES_KEEP_HOURS not set, using default of 24 hours");
                    24
                }),
            cleanup_interval: std::env::var("TEMPFILES_CLEANUP_INTERVAL")
                .map(|s| s.parse().unwrap())
                .unwrap_or_else(|_| {
                    rocket::warn!(
                        "TEMPFILES_CLEANUP_INTERVAL not set, using default of 60 seconds"
                    );
                    60
                }),
            cleanup_key: std::env::var("TEMPFILES_CLEANUP_KEY").unwrap_or_else(|_| {
                rocket::warn!("TEMPFILES_CLEANUP_KEY not set, generating random key");
                use rand::Rng;
                rand::thread_rng()
                    .sample_iter(&rand::distributions::Alphanumeric)
                    .take(32)
                    .map(char::from)
                    .collect()
            }),
        }
    }
}
