use std::fmt::Display;

use bon::Builder;

// use discord_game_sdk::Activity;
// use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Builder, Serialize, Deserialize)]
pub struct Game {
    name: String,
    #[serde(default = "Game::missing_platform")]
    platform: String,
    large_image_key: Option<String>,
    large_image_tooltip: Option<String>,
    small_image_key: Option<String>,
    small_image_tooltip: Option<String>,
}

impl From<Game> for ratatui::widgets::ListItem<'_> {
    fn from(game: Game) -> Self {
        game.two_line_view().into()
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
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.name, self.platform)
    }
}

impl Game {
    fn missing_platform() -> String {
        String::from("No platform")
    }

    pub fn two_line_view(&self) -> String {
        format!("{}\n{}", self.name, self.platform)
    }

    // pub fn generate_activity(&mut self) {
    //     let mut activity = Activity::empty()
    //         .with_state(&self.name)
    //         .with_details(&self.platform)
    //         .to_owned();

    //     let activity = match (
    //         &self.large_image_key,
    //         &self.large_image_tooltip,
    //         &self.small_image_key,
    //         &self.small_image_tooltip,
    //     ) {
    //         (None, None, None, None) => &mut activity,
    //         (None, None, None, Some(small_tooltip)) => {
    //             activity.with_small_image_tooltip(small_tooltip)
    //         }
    //         (None, None, Some(small_key), None) => activity.with_small_image_key(small_key),
    //         (None, None, Some(small_key), Some(small_tooltip)) => activity
    //             .with_small_image_key(small_key)
    //             .with_small_image_tooltip(small_tooltip),
    //         (None, Some(large_tooltip), None, None) => {
    //             activity.with_large_image_tooltip(large_tooltip)
    //         }
    //         (None, Some(large_tooltip), None, Some(small_tooltip)) => activity
    //             .with_large_image_tooltip(large_tooltip)
    //             .with_small_image_tooltip(small_tooltip),
    //         (None, Some(large_tooltip), Some(small_key), None) => activity
    //             .with_large_image_tooltip(large_tooltip)
    //             .with_small_image_key(small_key),
    //         (None, Some(large_tooltip), Some(small_key), Some(small_tooltip)) => activity
    //             .with_large_image_tooltip(large_tooltip)
    //             .with_small_image_key(small_key)
    //             .with_small_image_tooltip(small_tooltip),
    //         (Some(large_key), None, None, None) => activity.with_large_image_key(large_key),
    //         (Some(large_key), None, None, Some(small_tooltip)) => activity
    //             .with_large_image_key(large_key)
    //             .with_small_image_tooltip(small_tooltip),
    //         (Some(large_key), None, Some(small_key), None) => activity
    //             .with_large_image_key(large_key)
    //             .with_small_image_key(small_key),
    //         (Some(large_key), None, Some(small_key), Some(small_tooltip)) => activity
    //             .with_large_image_key(large_key)
    //             .with_small_image_key(small_key)
    //             .with_small_image_tooltip(small_tooltip),
    //         (Some(large_key), Some(large_tooltip), None, None) => activity
    //             .with_large_image_key(large_key)
    //             .with_large_image_tooltip(large_tooltip),
    //         (Some(large_key), Some(large_tooltip), None, Some(small_tooltip)) => activity
    //             .with_large_image_key(large_key)
    //             .with_large_image_tooltip(large_tooltip)
    //             .with_small_image_tooltip(small_tooltip),
    //         (Some(large_key), Some(large_tooltip), Some(small_key), None) => activity
    //             .with_large_image_key(large_key)
    //             .with_large_image_tooltip(large_tooltip)
    //             .with_small_image_key(small_key),
    //         (Some(large_key), Some(large_tooltip), Some(small_key), Some(small_tooltip)) => {
    //             activity
    //                 .with_large_image_key(large_key)
    //                 .with_large_image_tooltip(large_tooltip)
    //                 .with_small_image_key(small_key)
    //                 .with_small_image_tooltip(small_tooltip)
    //         }
    //     }
    //     .to_owned();

    //     self.activity = Some(activity);
    // }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn platform(&self) -> &str {
        &self.platform
    }

    pub const fn large_image_key(&self) -> Option<&String> {
        self.large_image_key.as_ref()
    }

    pub const fn large_image_tooltip(&self) -> Option<&String> {
        self.large_image_tooltip.as_ref()
    }

    pub const fn small_image_key(&self) -> Option<&String> {
        self.small_image_key.as_ref()
    }

    pub const fn small_image_tooltip(&self) -> Option<&String> {
        self.small_image_tooltip.as_ref()
    }
}

impl From<&Game> for discord_game_sdk::Activity {
    fn from(game: &Game) -> Self {
        use discord_game_sdk::Activity;

        let mut activity = Activity::empty()
            .with_state(&game.name)
            .with_details(&game.platform)
            .to_owned();

        match (
            &game.large_image_key,
            &game.large_image_tooltip,
            &game.small_image_key,
            &game.small_image_tooltip,
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
        .to_owned()
    }
}

impl From<Game> for discord_game_sdk::Activity {
    fn from(game: Game) -> Self {
        use discord_game_sdk::Activity;

        let mut activity = Activity::empty()
            .with_state(&game.name)
            .with_details(&game.platform)
            .to_owned();

        match (
            &game.large_image_key,
            &game.large_image_tooltip,
            &game.small_image_key,
            &game.small_image_tooltip,
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
        .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Game;
    use discord_game_sdk::Activity;
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

        let small_image = Game::builder()
            .name(name.clone())
            .small_image_key(small_key.clone())
            .small_image_tooltip(small_tooltip.clone())
            .platform(platform.clone())
            .build();
        let small_image_activity = Activity::from(&small_image);

        let large_image = Game::builder()
            .name(name.clone())
            .large_image_key(large_key.clone())
            .large_image_tooltip(large_tooltip.clone())
            .platform(platform.clone())
            .build();

        let large_image_activity = Activity::from(&large_image);

        assert_str_eq!(name, small_image.name);
        assert_str_eq!(name, small_image_activity.state());
        assert_str_eq!(small_key, small_image_activity.small_image_key());
        assert_str_eq!(small_tooltip, small_image_activity.small_image_tooltip());

        assert_str_eq!(name, large_image.name);
        assert_str_eq!(name, large_image_activity.state());
        assert_str_eq!(large_key, large_image_activity.large_image_key());
        assert_str_eq!(large_tooltip, large_image_activity.large_image_tooltip());
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
        // assert_eq!(default.activity, None);
    }
}
