use std::fs:: { File, OpenOptions };
use std::io:: { Write, Result };
use std::path:: { Path, PathBuf };
use log:: { Log, LogRecord, LogLevel, LogMetadata };

pub struct FileLogger(PathBuf);

impl FileLogger {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        FileLogger(path.as_ref().to_path_buf())
    }
    pub fn append(&self, record: &LogRecord) {
        let mut f = match log_file(self.0.as_path()) {
            Ok(f) => f,
            Err(err) => panic!("{:?}", err)
        };
        let _ = f.write_fmt(format_args!("{} - {}\n", record.level(), record.args()));
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

fn log_file<P: AsRef<Path>>(path: P) -> Result<File> {
    OpenOptions::new().write(true).append(true).open(path)
}
