use super::config::{LogConfig, LogLevel};
use bevy::prelude::*;
use chrono::Local;
use colored::*;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

/// 日志记录器资源
#[derive(Resource)]
pub struct GameLogger {
    log_file: Option<File>,
    config: LogConfig,
}

impl GameLogger {
    pub fn new(config: LogConfig) -> Self {
        let log_file = if config.file_output {
            Some(Self::create_log_file(&config.log_dir))
        } else {
            None
        };

        Self { log_file, config }
    }

    fn create_log_file(log_dir: &str) -> File {
        fs::create_dir_all(log_dir).expect("Failed to create log directory");
        let date = Local::now().format("%Y-%m-%d");
        let log_path = Path::new(log_dir).join(format!("{}.log", date));

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)
            .expect("Failed to open log file")
    }

    pub fn log(&mut self, level: LogLevel, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");

        // 控制台输出
        if self.config.console_output && level <= self.config.min_console_level {
            let colored_level = match level {
                LogLevel::Error => level.as_str().red().bold(),
                LogLevel::Info => level.as_str().green(),
                LogLevel::Debug => level.as_str().yellow(),
                LogLevel::Verbose => level.as_str().blue(),
            };

            let colored_message = match level {
                LogLevel::Error => message.red(),
                LogLevel::Info => message.white(),
                LogLevel::Debug => message.yellow(),
                LogLevel::Verbose => message.blue(),
            };

            println!(
                "[{}] [{}] {}",
                timestamp.to_string().white(),
                colored_level,
                colored_message
            );
        }

        // 文件输出
        if self.config.file_output && level <= self.config.min_file_level {
            if let Some(file) = &mut self.log_file {
                let entry = format!("[{}] [{}] {}\n", timestamp, level.as_str(), message);
                file.write_all(entry.as_bytes())
                    .expect("Failed to write to log file");
                file.flush().expect("Failed to flush log file");
            }
        }
    }
}
