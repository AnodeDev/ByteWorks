use flexi_logger::{FileSpec, Logger, WriteMode};

pub fn init_logging() {
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(FileSpec::default().directory("logs"))
        .write_mode(WriteMode::BufferAndFlush)
        .start()
        .unwrap();
}
