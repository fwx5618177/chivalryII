use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowSettings {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub render_distance: u32,
    pub shadow_quality: String,
    pub particle_limit: u32,
    pub debug_rendering: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicsSettings {
    pub timestep: f32,
    pub gravity: f32,
    pub debug_draw: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkSettings {
    pub tick_rate: u32,
    pub interpolation_delay: f32,
    pub debug_overlay: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub file_output: bool,
    pub console_output: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub window: WindowSettings,
    pub graphics: GraphicsSettings,
    pub physics: PhysicsSettings,
    pub network: NetworkSettings,
    pub logging: LoggingSettings,
}

impl GameSettings {
    /// 从指定路径加载游戏配置
    pub fn load(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_file = fs::read_to_string(config_path)?;
        let settings: GameSettings = serde_json::from_str(&config_file)?;
        Ok(settings)
    }

    /// 加载调试配置
    pub fn load_debug() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load("src/config/debug/game_settings.json")
    }

    /// 加载开发配置
    pub fn load_dev() -> Result<Self, Box<dyn std::error::Error>> {
        Self::load("src/config/dev/game_settings.json")
    }
}

pub enum ConfigType {
    Debug,
    Dev,
}

pub struct ConfigManager {
    settings: GameSettings,
}

impl ConfigManager {
    pub fn new(config_type: ConfigType) -> Result<Self, Box<dyn std::error::Error>> {
        let settings = match config_type {
            ConfigType::Debug => GameSettings::load_debug()?,
            ConfigType::Dev => GameSettings::load_dev()?,
        };
        Ok(Self { settings })
    }

    pub fn get_settings(&self) -> &GameSettings {
        &self.settings
    }
} 