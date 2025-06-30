use std::{
    env,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use log::debug;
use serde::{Deserialize, Serialize};

use crate::{
    error::GameError,
    game::{Game, gamelist::GameList},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct GameFile {
    #[serde(skip)]
    path: Option<PathBuf>,
    #[serde(default)]
    game: Vec<Game>,
}

impl GameFile {
    const CONFIG_FILE: &str = ".config/consoleplayergames.toml";

    pub fn new() -> Result<Self, GameError> {
        let path = Self::get_path()?;
        debug!("config path: {path:#?}");

        if !path.try_exists()? {
            debug!("creating config file");
            let _ = File::create(&path)?;
        }

        let mut file = File::open(&path)?;
        let path = Some(path);

        let mut content = String::new();
        let size = file.read_to_string(&mut content)?;
        debug!("Readed {size} bytes from the config file");

        let mut game_file: GameFile = toml::from_str(content.as_str())?;
        game_file.path = path;

        Ok(game_file)
    }

    fn get_path() -> Result<PathBuf, GameError> {
        debug!("reading $HOME");
        let home = env::var("HOME")?;
        debug!("$HOME content: {home}");

        let mut path = PathBuf::new();
        path.push(home);
        path.push(Self::CONFIG_FILE);

        Ok(path)
    }

    #[expect(dead_code)]
    pub(crate) fn game_iter(&self) -> std::slice::Iter<'_, Game> {
        self.game.iter()
    }

    pub(crate) fn owned_game_iter(&self) -> std::vec::IntoIter<Game> {
        self.game.clone().into_iter()
    }

    pub(crate) fn write(&self) -> Result<(), GameError> {
        let string: String = toml::to_string_pretty(self)?;
        let path = if let Some(path) = &self.path {
            path.clone()
        } else {
            Self::get_path()?
        };

        let mut file = File::open(path)?;
        file.write_all(string.as_bytes())?;

        Ok(())
    }
}

impl TryFrom<&GameList> for GameFile {
    type Error = GameError;

    fn try_from(list: &GameList) -> Result<Self, Self::Error> {
        let path = Self::get_path()?;
        debug!("config path: {path:#?}");

        if !path.try_exists()? {
            debug!("creating config file");
            let _ = File::create(&path)?;
        }

        let path = Some(path);

        Ok(Self {
            path,
            game: list.clone_inner(),
        })
    }
}
