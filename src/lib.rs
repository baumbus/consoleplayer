#![warn(clippy::missing_const_for_fn)]
#![feature(if_let_guard)]

use dotenv::dotenv;

use crate::app::App;

pub use crate::error::Error;

mod app;
mod error;
mod event_handler;
mod game;
mod tui;

pub mod config;
pub mod domain;
pub mod inbound;
pub mod outbound;

#[derive(Debug, Parser, Hash, PartialEq, Eq, Clone, Copy)]
struct Config {
    #[arg(short, long)]
    timestamp: bool,
}

use clap::Parser;

pub fn run() -> color_eyre::Result<()> {
    color_eyre::install()?;

    tui_logger::init_logger(log::LevelFilter::Debug).unwrap();

    tui_logger::set_default_level(log::LevelFilter::Trace);
    dotenv().ok();
    let config = Config::parse();

    let mut terminal = ratatui::init();
    let result = App::new()?.run(&mut terminal, config);
    ratatui::try_restore()?;
    result?;

    // let mut game = Game::builder().name("Mario & Luigi Brothership".into()).large_image_key("marioandluigibrothership".into()).build();

    Ok(())
}
