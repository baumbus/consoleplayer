use ratatui::{
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListState, StatefulWidget},
};

use crate::game::Game;

#[derive(Debug, Default)]
pub(crate) struct GameList {
    pub(crate) items: Vec<Game>,
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
            "<ENTER>".blue().bold(),
            " Navigation ".into(),
            "<Arrowkeys>".blue().bold(),
            " Edit ".into(),
            "<E>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
            " Unselect ".into(),
            "<U> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let items: Vec<Game> = self.items.to_vec();

        let list = List::new(items)
            .block(block)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(Style::new().light_blue().italic());

        StatefulWidget::render(list, area, buf, state);
    }
}
