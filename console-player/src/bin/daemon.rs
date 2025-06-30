use console_player::daemon::Daemon;
use tracing::error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tracing::instrument]
fn main() -> Result<(), console_player::daemon::error::DaemonError> {
    let registry =
        tracing_subscriber::registry().with(tracing_subscriber::fmt::layer().with_target(false));

    match tracing_journald::layer() {
        Ok(layer) => {
            registry.with(layer).init();
        }
        // journald is typically available on Linux systems, but nowhere else. Portable software
        // should handle its absence gracefully.
        Err(e) => {
            registry.init();
            error!("couldn't connect to journald: {}", e);
        }
    }

    let mut daemon = Daemon::new(None)?;

    match daemon.run() {
        Ok(_) => {}
        Err(err) => error!("{err}"),
    }

    Ok(())
}
