use log::info;
use ratatui::widgets::ListState;

use crate::game::{Game, gamefile::GameFile, gamelist::GameList};

pub struct Data {
    gamefile: GameFile,
    gamelist: GameList,
    list_state: ListState,
}

impl Data {
    pub(super) fn select(&mut self, index: Option<usize>) {
        self.list_state.select(index);
    }

    #[expect(dead_code)]
    pub(super) fn select_first(&mut self) {
        self.list_state.select_first();
    }

    #[expect(dead_code)]
    pub(super) fn select_last(&mut self) {
        self.list_state.select_last();
    }

    #[expect(dead_code)]
    pub(super) fn select_next(&mut self) {
        self.list_state.select_next();
    }

    #[expect(dead_code)]
    pub(super) fn select_none(&mut self) {
        self.select(None);
    }

    #[expect(dead_code)]
    pub(super) fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    #[expect(dead_code)]
    pub(super) fn delete_selected(&mut self) {
        if let Some(index) = self.list_state.selected() {
            let game = self.gamelist.remove(index);
            info!("Deleted: {game:?}");
        }
    }

    #[expect(dead_code)]
    #[inline]
    pub(super) fn selected_game(&self) -> Option<Game> {
        self.gamelist.get(self.list_state.selected()?).cloned()
    }

    #[expect(dead_code)]
    pub(super) fn update_gamefile(&mut self) -> Result<(), crate::Error> {
        self.gamefile = GameFile::try_from(&self.gamelist)?;
        self.gamefile.write()?;

        Ok(())
    }
}
