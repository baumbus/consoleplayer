#[derive(Debug, Default)]
pub enum State {
    #[default]
    Selection,
    Editing,
    Adding,
}