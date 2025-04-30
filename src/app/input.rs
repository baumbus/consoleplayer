use ratatui::crossterm::event::{Event, KeyCode};
use tui_input::{Input as TInput, backend::crossterm::EventHandler};

use crate::game::Game;

#[derive(Debug, Default, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Hash)]
pub enum Selection {
    #[default]
    Name,
    Platform,
    LargeImageKey,
    LargeImageTooltip,
    SmallImageKey,
    SmallImageTooltip,
}

impl Selection {
    pub const fn next(&mut self) -> Self {
        match self {
            Self::Name => Self::Platform,
            Self::Platform => Self::LargeImageKey,
            Self::LargeImageKey => Self::LargeImageTooltip,
            Self::LargeImageTooltip => Self::SmallImageKey,
            Self::SmallImageKey => Self::SmallImageTooltip,
            Self::SmallImageTooltip => Self::Name,
        }
    }

    pub const fn previous(&mut self) -> Self {
        match self {
            Self::Name => Self::SmallImageTooltip,
            Self::Platform => Self::Name,
            Self::LargeImageKey => Self::Platform,
            Self::LargeImageTooltip => Self::LargeImageKey,
            Self::SmallImageKey => Self::LargeImageTooltip,
            Self::SmallImageTooltip => Self::SmallImageKey,
        }
    }
}

#[derive(Debug, Default)]
pub struct Input {
    game: TInput,
    platform: TInput,
    large_image_key: TInput,
    large_image_tooltip: TInput,
    small_image_key: TInput,
    small_image_tooltip: TInput,
    current_selection: Selection,
}

impl Input {
    pub fn large_image_key(&self) -> Option<String> {
        if self.large_image_key.value().is_empty() {
            None
        } else {
            self.large_image_key.to_string().into()
        }
    }

    pub fn large_image_tooltip(&self) -> Option<String> {
        if self.large_image_tooltip.value().is_empty() {
            None
        } else {
            self.large_image_tooltip.to_string().into()
        }
    }

    pub fn small_image_key(&self) -> Option<String> {
        if self.small_image_key.value().is_empty() {
            None
        } else {
            self.small_image_key.to_string().into()
        }
    }

    pub fn small_image_tooltip(&self) -> Option<String> {
        if self.small_image_tooltip.value().is_empty() {
            None
        } else {
            self.small_image_tooltip.to_string().into()
        }
    }

    pub fn reset(&mut self) {
        self.game = Default::default();
        self.platform = Default::default();
        self.large_image_key = Default::default();
        self.large_image_tooltip = Default::default();
        self.small_image_key = Default::default();
        self.small_image_tooltip = Default::default();
        self.current_selection = Default::default();
    }
}

impl From<&Input> for Game {
    fn from(input: &Input) -> Self {
        Self::builder()
            .name(input.game.to_string())
            .platform(input.platform.to_string())
            .maybe_large_image_key(input.large_image_key())
            .maybe_large_image_tooltip(input.large_image_tooltip())
            .maybe_small_image_key(input.small_image_key())
            .maybe_small_image_tooltip(input.small_image_tooltip())
            .build()
    }
}

impl EventHandler for Input {
    fn handle_event(
        &mut self,
        evt: &ratatui::crossterm::event::Event,
    ) -> Option<tui_input::StateChanged> {
        match evt {
            Event::Key(key) if key.code == KeyCode::Tab => {
                self.current_selection = self.current_selection.next()
            }
            Event::Key(key) if key.code == KeyCode::BackTab => {
                self.current_selection = self.current_selection.previous()
            }
            _ => {}
        }

        match self.current_selection {
            Selection::Name => self.game.handle_event(evt),
            Selection::Platform => self.platform.handle_event(evt),
            Selection::LargeImageKey => self.large_image_key.handle_event(evt),
            Selection::LargeImageTooltip => self.large_image_tooltip.handle_event(evt),
            Selection::SmallImageKey => self.small_image_key.handle_event(evt),
            Selection::SmallImageTooltip => self.small_image_tooltip.handle_event(evt),
        }
    }
}
