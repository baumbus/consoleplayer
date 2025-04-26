use std::{env, fs::File, io::Read, path::PathBuf};

use bon::Builder;

use discord_game_sdk::Activity;
use log::debug;
use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};

use crate::error::GameError;

#[derive(Debug, Clone, PartialEq, Eq, Builder, Serialize, Deserialize)]
pub struct Game {
    name: String,
    #[serde(default = "Game::missing_platform")]
    platform: String,
    large_image_key: Option<String>,
    large_image_tooltip: Option<String>,
    small_image_key: Option<String>,
    small_image_tooltip: Option<String>,
    #[serde(skip)]
    pub(crate) activity: Option<Activity>,
}

impl From<Game> for ListItem<'_> {
    fn from(value: Game) -> Self {
        let view_string = format!("{}\n{}", value.name, value.platform);

        view_string.into()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            name: String::from("Idle"),
            platform: Game::missing_platform(),
            large_image_key: Default::default(),
            large_image_tooltip: Default::default(),
            small_image_key: Default::default(),
            small_image_tooltip: Default::default(),
            activity: Default::default(),
        }
    }
}

impl Game {
    fn missing_platform() -> String {
        String::from("No platform")
    }

    pub fn generate_activity(&mut self) {
        let mut activity = Activity::empty().with_state(&self.name).with_details(&self.platform).to_owned();

        let activity = match (
            &self.large_image_key,
            &self.large_image_tooltip,
            &self.small_image_key,
            &self.small_image_tooltip,
        ) {
            (None, None, None, None) => &mut activity,
            (None, None, None, Some(small_tooltip)) => {
                activity.with_small_image_tooltip(small_tooltip)
            }
            (None, None, Some(small_key), None) => activity.with_small_image_key(small_key),
            (None, None, Some(small_key), Some(small_tooltip)) => activity
                .with_small_image_key(small_key)
                .with_small_image_tooltip(small_tooltip),
            (None, Some(large_tooltip), None, None) => {
                activity.with_large_image_tooltip(large_tooltip)
            }
            (None, Some(large_tooltip), None, Some(small_tooltip)) => activity
                .with_large_image_tooltip(large_tooltip)
                .with_small_image_tooltip(small_tooltip),
            (None, Some(large_tooltip), Some(small_key), None) => activity
                .with_large_image_tooltip(large_tooltip)
                .with_small_image_key(small_key),
            (None, Some(large_tooltip), Some(small_key), Some(small_tooltip)) => activity
                .with_large_image_tooltip(large_tooltip)
                .with_small_image_key(small_key)
                .with_small_image_tooltip(small_tooltip),
            (Some(large_key), None, None, None) => activity.with_large_image_key(large_key),
            (Some(large_key), None, None, Some(small_tooltip)) => activity
                .with_large_image_key(large_key)
                .with_small_image_tooltip(small_tooltip),
            (Some(large_key), None, Some(small_key), None) => activity
                .with_large_image_key(large_key)
                .with_small_image_key(small_key),
            (Some(large_key), None, Some(small_key), Some(small_tooltip)) => activity
                .with_large_image_key(large_key)
                .with_small_image_key(small_key)
                .with_small_image_tooltip(small_tooltip),
            (Some(large_key), Some(large_tooltip), None, None) => activity
                .with_large_image_key(large_key)
                .with_large_image_tooltip(large_tooltip),
            (Some(large_key), Some(large_tooltip), None, Some(small_tooltip)) => activity
                .with_large_image_key(large_key)
                .with_large_image_tooltip(large_tooltip)
                .with_small_image_tooltip(small_tooltip),
            (Some(large_key), Some(large_tooltip), Some(small_key), None) => activity
                .with_large_image_key(large_key)
                .with_large_image_tooltip(large_tooltip)
                .with_small_image_key(small_key),
            (Some(large_key), Some(large_tooltip), Some(small_key), Some(small_tooltip)) => {
                activity
                    .with_large_image_key(large_key)
                    .with_large_image_tooltip(large_tooltip)
                    .with_small_image_key(small_key)
                    .with_small_image_tooltip(small_tooltip)
            }
        }
        .to_owned();

        self.activity = Some(activity);
    }
}

#[derive(Debug, Deserialize)]
pub struct GameFile {
    #[serde(skip)]
    path: Option<PathBuf>,
    #[serde(default)]
    game: Vec<Game>,
}

impl GameFile {
    const CONFIG_FILE: &str = ".config/consoleplayergames.toml";

    pub fn new() -> Result<Self, GameError> {
        debug!("reading $HOME");
        let home = env::var("HOME")?;
        debug!("$HOME content: {}", home);

        let mut path = PathBuf::new();
        path.push(home);
        path.push(Self::CONFIG_FILE);

        debug!("config path: {:#?}", path);

        if !path.try_exists()? {
            debug!("creating config file");
            let _ = File::create(&path)?;
        }

        let mut file = File::open(&path)?;
        let path = Some(path);

        let mut content = String::new();
        let size = file.read_to_string(&mut content)?;
        debug!("Readed {} bytes from the config file", size);

        let mut game_file: GameFile = toml::from_str(content.as_str())?;
        game_file.path = path;

        Ok(game_file)
    }

    #[expect(dead_code)]
    pub(crate) fn game_iter(&self) -> std::slice::Iter<'_, Game> {
        self.game.iter()
    }

    pub(crate) fn owned_game_iter(&self) -> std::vec::IntoIter<Game> {
        self.game.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Game;
    #[expect(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

    #[test]
    fn generate_activity_test() {
        let name = String::from("mario");
        let small_key = String::from("small_key");
        let small_tooltip = String::from("small_tooltip");
        let large_key = String::from("large_key");
        let large_tooltip = String::from("large_tooltip");
        let platform = String::from("Nintendo Switch");

        let mut small_image = Game::builder()
            .name(name.clone())
            .small_image_key(small_key.clone())
            .small_image_tooltip(small_tooltip.clone())
            .platform(platform.clone())
            .build();
        small_image.generate_activity();

        let mut large_image = Game::builder()
            .name(name.clone())
            .large_image_key(large_key.clone())
            .large_image_tooltip(large_tooltip.clone())
            .platform(platform.clone())
            .build();
        large_image.generate_activity();

        assert_str_eq!(name, small_image.name);
        assert_str_eq!(name, small_image.activity.clone().unwrap().state());
        assert_str_eq!(
            small_key,
            small_image.activity.clone().unwrap().small_image_key()
        );
        assert_str_eq!(
            small_tooltip,
            small_image.activity.unwrap().small_image_tooltip()
        );

        assert_str_eq!(name, large_image.name);
        assert_str_eq!(name, large_image.activity.clone().unwrap().state());
        assert_str_eq!(
            large_key,
            large_image.activity.clone().unwrap().large_image_key()
        );
        assert_str_eq!(
            large_tooltip,
            large_image.activity.unwrap().large_image_tooltip()
        );
    }

    #[test]
    fn game_default() {
        let default = Game::default();

        assert_str_eq!(default.name, "Idle");
        assert_str_eq!(default.platform, "No platform");
        assert_eq!(default.large_image_key, None);
        assert_eq!(default.large_image_tooltip, None);
        assert_eq!(default.small_image_key, None);
        assert_eq!(default.small_image_tooltip, None);
        assert_eq!(default.activity, None);
    }
}
