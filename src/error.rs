use std::{env::VarError, num::ParseIntError};

use thiserror::Error;

pub type GameError = Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    VarError(#[from] VarError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    TomlError(#[from] toml::de::Error),
    #[error(transparent)]
    DotEnv(#[from] dotenv::Error),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error(transparent)]
    Discord(#[from] discord_game_sdk::Error),
}
