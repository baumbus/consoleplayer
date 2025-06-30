use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use ratatui::{
    style::{Style, Stylize},
    symbols::border,
    widgets::{Block, List, ListState, StatefulWidget},
};
use tracing::{debug, trace};

use crate::game::Game;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GameWidget {
    items: Vec<Game>,
    #[serde(skip)]
    state: ListState,
    #[serde(skip)]
    path: Option<PathBuf>,
}

impl GameWidget {
    const CONFIG_FILE: &str = ".config/consoleplayergames.toml";

    fn get_path() -> Result<PathBuf, crate::error::Error> {
        debug!("reading $HOME");
        let home = std::env::var("HOME")?;
        debug!("$HOME content: {home}");

        let mut path = PathBuf::new();
        path.push(home);
        path.push(Self::CONFIG_FILE);

        if !path.try_exists()? {
            debug!("creating config file");
            let _ = File::create(&path)?;
        }

        Ok(path)
    }

    pub fn game_iter(&self) -> std::slice::Iter<'_, Game> {
        self.items.iter()
    }

    pub fn owned_game_iter(&self) -> std::vec::IntoIter<Game> {
        self.items.clone().into_iter()
    }

    pub fn write(&self) -> Result<(), crate::error::Error> {
        let string: String = toml::to_string_pretty(self).map_err(crate::error::Toml::from)?;
        let path = if let Some(path) = &self.path {
            path.clone()
        } else {
            Self::get_path()?
        };

        let mut file = File::open(path)?;
        file.write_all(string.as_bytes())?;

        Ok(())
    }

    pub fn render(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = ratatui::text::Line::from(" Games ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let items: Vec<Game> = self.items.to_vec();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .highlight_style(Style::new().light_blue().italic());

        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    pub fn builder() -> GameWidgetBuilder {
        GameWidgetBuilder::default()
    }
}

impl TryFrom<&Path> for GameWidget {
    type Error = crate::error::Error;

    #[tracing::instrument]
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        debug!("config path: {path:#?}");

        if !path.try_exists()? {
            debug!("creating config file");
            let _ = File::create(&path)?;
        }

        let mut file = File::open(&path)?;

        let mut content = String::new();
        let size = file.read_to_string(&mut content)?;
        debug!("Readed {size} bytes from the config file");

        let mut game_widget: Self =
            toml::from_str(content.as_str()).map_err(crate::error::Toml::from)?;
        game_widget.path = Some(path.to_path_buf());

        Ok(game_widget)
    }
}

#[derive(Debug, Default)]
pub struct GameWidgetBuilder {
    items: Vec<Game>,
    state: Option<ListState>,
    path: Option<PathBuf>,
}

impl GameWidgetBuilder {
    #[tracing::instrument]
    pub fn build(self) -> Result<GameWidget, crate::error::Error> {
        let items = self.items;

        let state = self.state.unwrap_or_default();

        let path = Some(if let Some(path) = self.path {
            path
        } else {
            GameWidget::get_path()?
        });

        trace!("Building GameWidget with: {items:?}; {state:?}; {path:?}");
        Ok(GameWidget { items, state, path })
    }

    pub fn games<T>(mut self, iter: T) -> Self
    where
        T: Iterator<Item = Game>,
    {
        self.items.extend(iter);
        self
    }

    pub const fn state(mut self, state: ListState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn path<T>(mut self, path: T) -> Self
    where
        T: Into<PathBuf>,
    {
        self.path = Some(path.into());
        self
    }
}
