/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error,   // 错误信息
    Info,    // 重要信息
    Debug,   // 调试信息
    Verbose, // 详细信息
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Verbose => "VERBOSE",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "error" => LogLevel::Error,
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "verbose" => LogLevel::Verbose,
            _ => LogLevel::Info,
        }
    }
}

/// 日志配置
#[derive(Clone)]
pub struct LogConfig {
    pub file_output: bool,      // 是否输出到文件
    pub console_output: bool,   // 是否输出到控制台
    pub min_file_level: LogLevel,    // 文件记录的最低级别
    pub min_console_level: LogLevel, // 控制台输出的最低级别
    pub log_dir: String,       // 日志文件目录
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            file_output: true,
            console_output: true,
            min_file_level: LogLevel::Info,
            min_console_level: LogLevel::Info,  // 默认控制台也是 Info 级别
            log_dir: "logs".to_string(),
        }
    }
} 