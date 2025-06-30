use std::os::unix::net::UnixStream;

#[derive(Debug)]
pub enum DaemonState {
    Startup,
    Connected(UnixStream),
    Disconnected,
    Exit,
}
