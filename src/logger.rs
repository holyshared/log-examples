use std::fs:: { File, OpenOptions };
use std::io::Write;
use std::path:: { Path, PathBuf };
use log:: { Log, LogRecord, LogLevel, LogMetadata, max_log_level, set_logger, LogLevelFilter, SetLoggerError };

pub struct SimpleLogger;
pub struct FileLogger(PathBuf);

impl Log for SimpleLogger {
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

impl FileLogger {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        FileLogger(path.as_ref().to_path_buf())
    }
    pub fn append(&self, record: &LogRecord) {
        let mut f = match OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.0.as_path()) {
            Ok(f) => f,
            Err(err) => panic!("{:?}", err)
        };
        f.write_fmt(format_args!("{} - {}", record.level(), record.args()));
    }
}

impl Log for FileLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }
    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) == false {
            return;
        }
        self.append(record);
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(FileLogger::new("./debug.log"))
    })
}
