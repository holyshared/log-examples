mod file_logger;
mod console_logger;

use log:: { max_log_level, set_logger, LogLevelFilter, SetLoggerError };
use self::file_logger::FileLogger;

pub fn init() -> Result<(), SetLoggerError> {
    set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Info);
        Box::new(FileLogger::new("./debug.log"))
    })
}
