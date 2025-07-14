use ratatui::
    DefaultTerminal
;

mod data;
pub mod input;
mod state;
mod tabs;
mod widgets;

#[expect(dead_code)]
pub struct Tui {
    selected_tab: tabs::Tabs,
    state: state::State,
    data: data::Data,
}

impl Tui {
    #[expect(dead_code)]
    pub fn run(&self, _terminal: DefaultTerminal) -> Result<(), ()> {
        todo!()
    }
}
