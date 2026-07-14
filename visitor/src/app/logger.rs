use tracing_appender::{non_blocking::{self, WorkerGuard}, rolling::{self}};
use tracing_subscriber::fmt;

pub struct Logger {
    guard: WorkerGuard
}

impl Logger {
    pub fn new() -> Logger {
        let path = dirs::data_dir().unwrap().join("visitor");
        let appender = rolling::daily(path, "visitor.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(appender);
        
        fmt()
            .with_writer(non_blocking)
            .with_ansi(false)
            .init();
        
        Logger { guard }
    }
}