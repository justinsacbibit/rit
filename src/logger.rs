extern crate log;

use log::{LogRecord, LogLevel, LogMetadata};

pub struct Logger {
    pub verbose: bool,
}
impl log::Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Warn
            || self.verbose && metadata.level() == LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

