use std::io;

use discord_game_sdk::Discord;
use dotenv::var;
use log::info;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListState, StatefulWidget},
};

use crate::{
    Config,
    event_handler::EventHandler,
    game::{Game, GameFile},
};

#[derive(Debug, Default)]
pub(crate) struct GameList {
    items: Vec<Game>,
}

#[derive(Debug)]
pub(crate) struct App<'a> {
    list: GameList,
    exit: bool,
    list_state: ListState,
    discord: Discord<'a, EventHandler>,
    gamefile: GameFile,
}

impl FromIterator<Game> for GameList {
    fn from_iter<T: IntoIterator<Item = Game>>(iter: T) -> Self {
        let items = iter.into_iter().collect();

        Self { items }
    }
}

impl<'a> App<'a> {
    pub fn new() -> Result<Self, crate::Error> {
        Ok(Self {
            list: Default::default(),
            exit: Default::default(),
            list_state: Default::default(),
            discord: Self::init_discord()?,
            gamefile: GameFile::new()?,
        })
    }

    pub fn add_games<T: std::iter::IntoIterator<Item = Game>>(&mut self, games: T) {
        for game in games {
            self.list.items.push(game);
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        config: Config,
    ) -> Result<(), crate::error::Error> {
        info!("timestamp: {}", config.timestamp);

        self.add_games(self.gamefile.owned_game_iter());

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
            self.discord.run_callbacks()?;
        }
        Ok(())
    }

    fn init_discord() -> Result<Discord<'a, EventHandler>, crate::Error> {
        let client_id: i64 = var("CLIENT_ID")?.parse()?;

        info!("init discord");
        let mut discord = discord_game_sdk::Discord::new(client_id)?;
        *discord.event_handler_mut() = Some(crate::event_handler::EventHandler);

        Ok(discord)
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_stateful_widget(&self.list, frame.area(), &mut self.list_state);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            event::KeyCode::Char('q') => self.exit(),
            event::KeyCode::Char('u') => self.select_none(),
            event::KeyCode::Up => self.select_previous(),
            event::KeyCode::Down => self.select_next(),
            event::KeyCode::PageUp => self.select_first(),
            event::KeyCode::PageDown => self.select_last(),
            event::KeyCode::Enter => self.activate_current(),
            _ => {}
        }
    }

    const fn exit(&mut self) {
        self.exit = true;
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.list_state.select(index);
    }

    pub fn select_first(&mut self) {
        self.list_state.select_first();
    }

    pub fn select_last(&mut self) {
        self.list_state.select_last();
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }

    pub fn select_none(&mut self) {
        self.select(None);
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    #[inline]
    pub fn selected_game(&self) -> Option<Game> {
        self.list.items.get(self.list_state.selected()?).cloned()
    }

    pub fn activate_current(&mut self) {
        let mut game = match self.selected_game() {
            Some(game) => game.clone(),
            None => Game::default(),
        };

        game.generate_activity();
        let activity = game.activity;

        if let Some(ref activity) = activity {
            self.discord.update_activity(activity, |_discord, result| {
                if let Err(error) = result {
                    eprintln!("failed to update activity: {}", error);
                }
            });
        }
    }
}

impl StatefulWidget for &GameList {
    type State = ListState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let title = Line::from(" Games ".bold());
        let instructions = Line::from(vec![
            " Activate current selection ".into(),
            "<ENTER> ".blue().bold(),
            " Navigation ".into(),
            "<Arrowkeys> ".blue().bold(),
            " Edit ".into(),
            "<E> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Unselect ".into(),
            "<U> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<Game> = self.items.to_vec();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, state);
    }
}
