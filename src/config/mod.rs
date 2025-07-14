use directories::ProjectDirs;

pub struct Config {
    database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let project_dir = match ProjectDirs::from("de", "baumbus", "console-player") {
            Some(dir) => dir,
            None => return Err(std::env::VarError::NotPresent),
        };

        let database_url = match std::env::var("CONSOLE_PLAYER_DATABASE_URL") {
            Ok(url) => url,
            Err(e) if e.eq(&std::env::VarError::NotPresent) => {
                project_dir.data_dir().display().to_string()
            }
            Err(e) => return Err(e),
        };

        Ok(Self { database_url })
    }
}
