use std::io;

use discord_game_sdk::Discord;
use dotenv::var;
use log::info;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEventKind},
    widgets::ListState,
};
use tui_input::backend::crossterm::EventHandler as _;

use crate::{
    Config,
    event_handler::EventHandler,
    game::{Game, gamefile::GameFile, gamelist::GameList},
};

use crate::app::input::Input as AppInput;
use crate::app::state::State as AppState;

mod input;
mod state;

#[derive(Debug)]
pub(crate) struct App<'a> {
    list: GameList,
    exit: bool,
    list_state: ListState,
    discord: Discord<'a, EventHandler>,
    gamefile: GameFile,
    state: AppState,
    input: AppInput,
}

impl FromIterator<Game> for GameList {
    fn from_iter<T: IntoIterator<Item = Game>>(iter: T) -> Self {
        let items = iter.into_iter().collect();

        Self { items }
    }
}

impl<'a> App<'a> {
    pub(crate) fn new() -> Result<Self, crate::Error> {
        Ok(Self {
            list: Default::default(),
            exit: Default::default(),
            list_state: Default::default(),
            discord: Self::init_discord()?,
            gamefile: GameFile::new()?,
            state: AppState::default(),
            input: AppInput::default(),
        })
    }

    fn add_games<T: std::iter::IntoIterator<Item = Game>>(&mut self, games: T) {
        for game in games {
            self.list.items.push(game);
        }
    }

    fn add_game(&mut self, game: Game) {
        self.list.items.push(game);
    }

    fn replace_current_selection(&mut self, game: Game) -> Option<usize> {
        let selected = self.list_state.selected()?;

        let selected_game = self.list.items.get_mut(selected)?;

        *selected_game = game;

        Some(selected)
    }

    pub(crate) fn run(
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
        let event = event::read()?;
        match event {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, event: Event) {
        match self.state {
            AppState::Selection => if let Event::Key(key_event) = event { match key_event.code {
                event::KeyCode::Char('q') => self.exit(),
                event::KeyCode::Char('u') => self.select_none(),
                event::KeyCode::Char('e') if self.selected_game().is_some() => {
                    self.switch_state_to(AppState::Editing)
                }
                event::KeyCode::Char('a') => self.switch_state_to(AppState::Adding),
                event::KeyCode::Up => self.select_previous(),
                event::KeyCode::Down => self.select_next(),
                event::KeyCode::PageUp => self.select_first(),
                event::KeyCode::PageDown => self.select_last(),
                event::KeyCode::Enter => self.activate_current(),
                _ => {}
            } },
            AppState::Editing => match event {
                Event::Key(key_event) if key_event.code == event::KeyCode::Enter => {
                    self.replace_current_selection((&self.input).into());
                    self.state = AppState::Selection;
                },
                Event::Key(key_event) if key_event.code == event::KeyCode::Esc => {
                    self.input.reset();
                    self.state = AppState::Selection;
                },
                evt => {
                    self.input.handle_event(&evt);
                }
            },
            AppState::Adding => match event {
                Event::Key(key_event) if key_event.code == event::KeyCode::Enter => {
                    self.add_game((&self.input).into());
                    self.state = AppState::Selection;
                },
                Event::Key(key_event) if key_event.code == event::KeyCode::Esc => {
                    self.input.reset();
                    self.state = AppState::Selection;
                },
                evt => {
                    self.input.handle_event(&evt);
                }
            },
        }
    }

    const fn exit(&mut self) {
        self.exit = true;
    }

    fn select(&mut self, index: Option<usize>) {
        self.list_state.select(index);
    }

    fn select_first(&mut self) {
        self.list_state.select_first();
    }

    fn select_last(&mut self) {
        self.list_state.select_last();
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
    }

    fn select_none(&mut self) {
        self.select(None);
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    const fn switch_state_to(&mut self, state: AppState) {
        self.state = state;
    }

    #[inline]
    fn selected_game(&self) -> Option<Game> {
        self.list.items.get(self.list_state.selected()?).cloned()
    }

    fn activate_current(&mut self) {
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
