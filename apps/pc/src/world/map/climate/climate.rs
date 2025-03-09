use std::collections::HashMap;

/// 气候配置系统
///
/// # 设计理念
/// 1. 模块化：各个气候要素独立配置
/// 2. 预设系统：提供常用的气候类型
/// 3. 灵活性：支持运行时动态调整
///
/// # 应用场景
/// 1. 不同地区的特色气候
/// 2. 特殊剧情的天气效果
/// 3. 动态事件的环境渲染
///
/// # 平衡考虑
/// 1. 基础参数影响整体游戏体验
/// 2. 变化范围决定游戏难度
/// 3. 天气系统增加游戏随机性
#[derive(Debug, Clone)]
pub struct Climate {
    /// 温度基准值
    pub base_temperature: f32,
    /// 温度变化范围
    pub temperature_range: f32,
    /// 温度噪声频率
    pub temperature_frequency: f32,

    /// 湿度基准值
    pub base_humidity: f32,
    /// 湿度变化范围
    pub humidity_range: f32,
    /// 湿度噪声频率
    pub humidity_frequency: f32,

    /// 风力基准值
    pub base_wind: f32,
    /// 风力变化范围
    pub wind_range: f32,
    /// 风力噪声频率
    pub wind_frequency: f32,
    /// 主导风向（弧度）
    pub wind_direction: f32,

    // 季节参数
    /// 是否启用季节变化
    pub enable_seasons: bool,
    /// 季节变化速度
    pub season_speed: f32,
    /// 季节影响强度
    pub season_strength: f32,

    // 天气参数
    /// 是否启用天气系统
    pub enable_weather: bool,
    /// 雨水概率
    pub rain_probability: f32,
    /// 雨水强度
    pub rain_intensity: f32,
    /// 雾气概率
    pub fog_probability: f32,
    /// 雾气密度
    pub fog_density: f32,
}

impl Default for Climate {
    fn default() -> Self {
        Self {
            base_temperature: 20.0,
            temperature_range: 10.0,
            temperature_frequency: 0.01,

            base_humidity: 0.5,
            humidity_range: 0.3,
            humidity_frequency: 0.02,

            base_wind: 1.0,
            wind_range: 2.0,
            wind_frequency: 0.05,
            wind_direction: 0.0, // 东风

            enable_seasons: true,
            season_speed: 1.0,
            season_strength: 0.8,

            enable_weather: true,
            rain_probability: 0.3,
            rain_intensity: 0.5,
            fog_probability: 0.2,
            fog_density: 0.4,
        }
    }
}

impl Climate {
    /// 创建新的气候配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建温暖湿润的气候配置
    pub fn warm_humid() -> Self {
        Self {
            base_temperature: 25.0,
            base_humidity: 0.8,
            rain_probability: 0.5,
            fog_probability: 0.3,
            ..Default::default()
        }
    }

    /// 创建寒冷干燥的气候配置
    pub fn cold_dry() -> Self {
        Self {
            base_temperature: 5.0,
            temperature_range: 15.0,
            base_humidity: 0.2,
            rain_probability: 0.1,
            ..Default::default()
        }
    }

    /// 创建多风的气候配置
    pub fn windy() -> Self {
        Self {
            base_wind: 3.0,
            wind_range: 4.0,
            wind_frequency: 0.1,
            ..Default::default()
        }
    }

    /// 创建多雾的气候配置
    pub fn foggy() -> Self {
        Self {
            fog_probability: 0.6,
            fog_density: 0.7,
            base_humidity: 0.7,
            ..Default::default()
        }
    }
}
