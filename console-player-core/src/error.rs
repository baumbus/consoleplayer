use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DotEnv(#[from] dotenv::Error),
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    DiscordGameSdk(#[from] discord_game_sdk::Error),
    #[error(transparent)]
    StdIo(#[from] std::io::Error),
    #[error(transparent)]
    Var(#[from] std::env::VarError),
    #[error(transparent)]
    Toml(#[from] Toml),
}

#[derive(Debug, Error)]
pub enum Toml {
    #[error(transparent)]
    Serialize(#[from] toml::ser::Error),
    #[error(transparent)]
    Deserialize(#[from] toml::de::Error),
}
