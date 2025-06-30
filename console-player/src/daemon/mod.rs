use std::fs::File;
use std::io::Read;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use console_player_core::discord::DiscordConnection;
use fd_lock::RwLock;
use tracing::debug;
use tracing::info;
use tracing::trace;

pub mod error;
pub mod state;

#[derive(Debug)]
pub struct Daemon<'a> {
    state: state::DaemonState,
    discord: DiscordConnection<'a>,
    client_socket: UnixListener,
    rw_lock: RwLock<File>,
}

impl<'a> Daemon<'a> {
    pub const SOCKET_PATH: &'static str = "/tmp/console-player.sock";
    pub const LOCKFILE_PATH: &'static str = "/var/lock/console-player.lock";

    #[tracing::instrument]
    pub fn new(id: Option<i64>) -> Result<Self, error::DaemonError> {
        if let Some(id) = id {
            let id_str = id.to_string();

            unsafe { std::env::set_var("CLIENT_ID", id_str) };
        }

        if std::fs::exists(Self::SOCKET_PATH)? {
            debug!("Socket already exists. Deleting existing one.");
            std::fs::remove_file(Self::SOCKET_PATH)?;
        }

        debug!("Connecting to unix socket at: {}", Self::SOCKET_PATH);
        let client_socket = UnixListener::bind(Self::SOCKET_PATH)?;
        client_socket.set_nonblocking(true)?;

        let path = Path::new(Self::LOCKFILE_PATH);

        let lock_file = if path.try_exists()? {
            File::open(path)?
        } else {
            File::create_new(path)?
        };

        let rw_lock = RwLock::new(lock_file);

        Ok(Self {
            state: state::DaemonState::Startup,
            discord: DiscordConnection::new()?,
            client_socket,
            rw_lock,
        })
    }

    pub fn run(&mut self) -> Result<(), error::DaemonError> {
        let _lock = match self.rw_lock.try_write() {
            Ok(guard) => guard,
            Err(err) => {
                trace!("{err}");
                return Err(error::DaemonError::DaemonAlreadyActiv);
            }
        };

        let term = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;

        self.state = state::DaemonState::Disconnected;

        loop {
            if term.load(Ordering::Relaxed) {
                info!("shutting down");
                break;
            }

            // TODO: Add check if another daemon is run

            match self.client_socket.accept() {
                Ok(connection) => {
                    debug!("Connected to {:?}", connection.1.as_pathname());
                    let mut stream = connection.0;

                    let mut buf = String::new();

                    stream.read_to_string(&mut buf)?;

                    debug!(received = buf.trim());
                }
                Err(_) => {},
            }
        }

        Ok(())
    }
}
