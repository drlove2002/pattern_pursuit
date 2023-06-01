use slog::{o, Drain, Logger};
use slog_term::{FullFormat, TermDecorator};

/// Configure the logger for the application
pub fn config() -> Logger {
    let decorator = TermDecorator::new().build();

    let console_drain = FullFormat::new(decorator)
        .use_file_location()
        .build()
        .fuse();

    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    Logger::root(console_drain, o!())
}
