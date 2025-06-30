use thiserror::Error;

#[derive(Debug, Error)]
pub enum DaemonError {
    #[error(transparent)]
    CoreError(#[from] console_player_core::error::Error),
    #[error(transparent)]
    StdIo(#[from] std::io::Error),
    #[error("Another daemon is already running")]
    DaemonAlreadyActiv,
}
