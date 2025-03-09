use crate::logging::{GameLogger, LogConfig};
use bevy::prelude::*;

/// 日志系统插件
pub struct LoggingPlugin;

impl Plugin for LoggingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameLogger::new(LogConfig::default()));
    }
}

impl Default for LoggingPlugin {
    fn default() -> Self {
        Self
    }
}
