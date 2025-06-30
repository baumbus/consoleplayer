#![warn(clippy::missing_const_for_fn)]
#![feature(if_let_guard)]

pub mod discord;
pub mod error;
pub mod game;
pub mod widgets;

#[cfg(test)]
mod tests {
    #[test]
    fn supported_platforms() {
        assert!(cfg!(unix));
    }
}
