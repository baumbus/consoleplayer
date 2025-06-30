#[derive(Debug, Default, PartialEq)]
pub enum State {
    #[default]
    Selection,
    Editing(crate::tui::input::Input),
    Adding(crate::tui::input::Input),
}

impl State {
    pub const fn is_adding(&self) -> bool {
        match self {
            State::Selection => false,
            State::Editing(_) => false,
            State::Adding(_) => true,
        }
    }

    #[expect(dead_code)]
    pub const fn is_selecting(&self) -> bool {
        match self {
            State::Selection => true,
            State::Editing(_) => false,
            State::Adding(_) => false,
        }
    }

    pub const fn is_editing(&self) -> bool {
        match self {
            State::Selection => false,
            State::Editing(_) => true,
            State::Adding(_) => false,
        }
    }
}
