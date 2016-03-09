//use std::fs:: { File, OpenOptions };
//use std::io:: { Write, Error };
//use std::path:: { Path, PathBuf };
use log:: { Log, LogRecord, LogLevel, LogMetadata };

pub struct ConsoleLogger;

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }
    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) == false {
            return;
        }
        println!("{} - {}", record.level(), record.args());
    }
}
