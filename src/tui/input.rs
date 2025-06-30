use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::Color,
    widgets::{Block, Paragraph},
};
use tui_input::{Input as TuiInput, backend::crossterm::EventHandler};

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
    game: TuiInput,
    platform: TuiInput,
    large_image_key: TuiInput,
    large_image_tooltip: TuiInput,
    small_image_key: TuiInput,
    small_image_tooltip: TuiInput,
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

    fn render_input(
        &self,
        text_input: &TuiInput,
        frame: &mut Frame,
        area: Rect,
        selection: Selection,
        title: &str,
    ) {
        // keep 2 for borders and 1 for cursor
        let width = area.width.max(3) - 3;
        let scroll = text_input.visual_scroll(width as usize);
        let style = if self.current_selection == selection {
            Color::Yellow
        } else {
            Color::White
        };
        let input = Paragraph::new(text_input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered().title(title));
        frame.render_widget(input, area);

        if self.current_selection == selection {
            // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
            // end of the input text and one line down from the border to the input line
            let x = text_input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }

    pub fn render(&self, area: Rect, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Min(3),
                Constraint::Min(3),
            ])
            .split(area);

        self.render_input(&self.game, frame, layout[0], Selection::Name, "Game");
        self.render_input(
            &self.platform,
            frame,
            layout[1],
            Selection::Platform,
            "Platform",
        );
        self.render_input(
            &self.large_image_key,
            frame,
            layout[2],
            Selection::LargeImageKey,
            "Large Image Key",
        );
        self.render_input(
            &self.large_image_tooltip,
            frame,
            layout[3],
            Selection::LargeImageTooltip,
            "Large Image Tooltip",
        );
        self.render_input(
            &self.small_image_key,
            frame,
            layout[4],
            Selection::SmallImageKey,
            "Small Image Key",
        );
        self.render_input(
            &self.small_image_tooltip,
            frame,
            layout[5],
            Selection::SmallImageTooltip,
            "Small Image Tooltip",
        );
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.game.value() == other.game.value()
            && self.platform.value() == other.platform.value()
            && self.large_image_key.value() == other.large_image_key.value()
            && self.large_image_tooltip.value() == other.large_image_tooltip.value()
            && self.small_image_key.value() == other.small_image_key.value()
            && self.small_image_tooltip.value() == other.small_image_tooltip.value()
            && self.current_selection == other.current_selection
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

impl From<&mut Input> for Game {
    fn from(input: &mut Input) -> Self {
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

impl From<Game> for Input {
    fn from(game: Game) -> Self {
        Self {
            game: TuiInput::new(game.name().into()),
            platform: TuiInput::new(game.platform().into()),
            large_image_key: TuiInput::new(
                game.large_image_key()
                    .unwrap_or(&String::from(""))
                    .to_string(),
            ),
            large_image_tooltip: TuiInput::new(
                game.large_image_tooltip()
                    .unwrap_or(&String::from(""))
                    .to_string(),
            ),
            small_image_key: TuiInput::new(
                game.small_image_key()
                    .unwrap_or(&String::from(""))
                    .to_string(),
            ),
            small_image_tooltip: TuiInput::new(
                game.small_image_tooltip()
                    .unwrap_or(&String::from(""))
                    .to_string(),
            ),
            current_selection: Selection::default(),
        }
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
