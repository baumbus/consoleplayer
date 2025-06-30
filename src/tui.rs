use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::Block,
};

mod data;
pub mod input;
mod state;
mod tabs;
mod widgets;

pub struct Tui {
    selected_tab: tabs::Tabs,
    state: state::State,
    data: data::Data,
}

impl Tui {
    #[expect(dead_code)]
    pub fn run(&self, terminal: DefaultTerminal) -> Result<(), ()> {
        todo!()
    }
}
