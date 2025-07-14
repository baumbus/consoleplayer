#[expect(dead_code)]
pub enum State {
    NotConnected,
    Selection,
    Adding(crate::tui::input::Input),
    Editing(crate::tui::input::Input),
    Quiting,
}

impl State {
    #[expect(dead_code)]
    pub const fn is_not_connected(&self) -> bool {
        match self {
            Self::NotConnected => true,
            Self::Selection => false,
            Self::Editing(_) => false,
            Self::Adding(_) => false,
            Self::Quiting => false,
        }
    }

    #[expect(dead_code)]
    pub const fn is_adding(&self) -> bool {
        match self {
            Self::NotConnected => false,
            Self::Selection => true,
            Self::Editing(_) => false,
            Self::Adding(_) => false,
            Self::Quiting => false,
        }
    }

    #[expect(dead_code)]
    pub const fn is_selecting(&self) -> bool {
        match self {
            Self::NotConnected => false,
            Self::Selection => false,
            Self::Editing(_) => true,
            Self::Adding(_) => false,
            Self::Quiting => false,
        }
    }

    #[expect(dead_code)]
    pub const fn is_editing(&self) -> bool {
        match self {
            Self::NotConnected => false,
            Self::Selection => false,
            Self::Editing(_) => false,
            Self::Adding(_) => true,
            Self::Quiting => false,
        }
    }

    #[expect(dead_code)]
    pub const fn is_quiting(&self) -> bool {
        match self {
            Self::NotConnected => false,
            Self::Selection => false,
            Self::Editing(_) => false,
            Self::Adding(_) => false,
            Self::Quiting => true,
        }
    }
}
