/// Simple logging to stdout.
///
/// Is basically the same as the example in the Rust book
/// https://doc.rust-lang.org/log/log/index.html
///
/// Information is printed with the macros debug!(), info!(), warn!(), error!().


use log;
use log::{LogRecord, LogMetadata, SetLoggerError, LogLevelFilter};

struct Logger;


impl log::Log for Logger {
    fn enabled(&self, _: &LogMetadata) -> bool   { true }
    fn log(&self, record: &LogRecord) {
        println!("{} - {}", record.level(), record.args());
    }
}

pub fn init(l: LogLevelFilter) -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(l);
        Box::new(Logger)
    })
}
