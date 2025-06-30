use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

pub struct Controls;

impl Controls {
    const TITLE: &str = "Controls";

    fn get_instruction() -> Line<'static> {
        Line::from(vec![
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
        ])
    }
}

impl Widget for Controls {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .border_type(ratatui::widgets::BorderType::Thick)
            .title(Self::TITLE);
        let paragraph = Paragraph::new(Self::get_instruction()).block(block);

        paragraph.render(area, buf);
    }
}
