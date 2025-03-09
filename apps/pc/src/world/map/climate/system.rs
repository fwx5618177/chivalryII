use bevy::utils::HashMap;
use noise::{NoiseFn, Perlin};

use super::{ClimateParams, Season, Zone};

/// 气候系统实现
///
/// # 核心功能
/// 1. 气候生成：创建合理的气候分布
/// 2. 动态变化：支持实时的天气和季节更替
/// 3. 环境影响：气候影响地形、植被和生物
/// 4. 游戏平衡：通过参数调整影响游戏难度
///
/// # 技术实现
/// 1. 噪声系统：使用多层柏林噪声创造自然变化
/// 2. 缓存机制：优化性能，避免重复计算
/// 3. 参数化：所有关键数值都可配置
///
/// # 性能考虑
/// 1. 使用缓存减少计算量
/// 2. 延迟计算非必要数据
/// 3. 优化数据结构和算法
#[derive(Debug, Clone)]
pub struct System {
    /// 气候参数
    pub params: ClimateParams,
    /// 温度噪声生成器
    temperature_noise: Perlin,
    /// 湿度噪声生成器
    moisture_noise: Perlin,
    /// 当前季节
    pub current_season: Season,
    /// 气候缓存
    climate_cache: HashMap<(i32, i32), (f32, f32)>, // (temperature, moisture)
    /// 种子
    pub seed: u64,
}

impl Default for System {
    fn default() -> Self {
        Self {
            params: ClimateParams::default(),
            temperature_noise: Perlin::new(1),
            moisture_noise: Perlin::new(2),
            current_season: Season::Summer,
            climate_cache: HashMap::new(),
            seed: 12345,
        }
    }
}

impl System {
    /// 初始化气候系统
    pub fn initialize(&mut self, seed: u64) {
        self.seed = seed;
        self.temperature_noise = Perlin::new(seed as u32);
        self.moisture_noise = Perlin::new((seed + 1) as u32);
        self.climate_cache.clear();
    }

    /// 设置当前季节
    pub fn set_season(&mut self, season: Season) {
        self.current_season = season;
        // 更改季节时清空缓存，因为气候条件会发生变化
        self.climate_cache.clear();
    }

    /// 获取指定位置的温度值 (0.0-1.0)
    pub fn get_temperature(&self, x: i32, y: i32) -> f32 {
        // 检查缓存
        if let Some(&(temp, _)) = self.climate_cache.get(&(x, y)) {
            return temp;
        }

        // 计算基础温度
        let nx = x as f64 * 0.02;
        let ny = y as f64 * 0.02;
        let base_temperature = self.temperature_noise.get([nx, ny]) as f32;

        // 纬度影响（北高南低）
        let world_height = 10000.0; // 假设世界总高度
        let latitude_factor = (y as f32 / world_height).min(1.0);
        let latitude_effect = (1.0 - latitude_factor) * self.params.latitude_temperature_factor;

        // 季节影响
        let season_effect = match self.current_season {
            Season::Summer => 0.2,
            Season::Spring | Season::Autumn => 0.0,
            Season::Winter => -0.2,
        };

        // 高度影响温度（假设外部会提供高度信息）
        // 这里简化实现，使用噪声模拟高度
        let alt_nx = x as f64 * 0.01;
        let alt_ny = y as f64 * 0.01;
        let simulated_height = (self.temperature_noise.get([alt_nx, alt_ny]) + 1.0) * 0.5;
        let altitude_effect = -simulated_height as f32 * self.params.altitude_temperature_factor;

        // 生成最终温度
        let temperature = ((base_temperature + 1.0) * 0.5 // 转换到0-1
            + latitude_effect
            + season_effect
            + altitude_effect)
            * self.params.temperature_scale
            + self.params.temperature_offset;

        // 标准化到0.0-1.0范围
        let normalized_temp = temperature.min(1.0).max(0.0);

        // 同时生成湿度
        let moisture = self.calculate_moisture(x, y);

        // 更新缓存
        let mut cache = self.climate_cache.clone();
        cache.insert((x, y), (normalized_temp, moisture));

        normalized_temp
    }

    /// 获取指定位置的湿度值 (0.0-1.0)
    pub fn get_moisture(&self, x: i32, y: i32) -> f32 {
        // 检查缓存
        if let Some(&(_, moisture)) = self.climate_cache.get(&(x, y)) {
            return moisture;
        }

        // 没有缓存，同时计算温度和湿度
        let temperature = self.get_temperature(x, y);

        // 通过缓存获取刚才计算的湿度
        if let Some(&(_, moisture)) = self.climate_cache.get(&(x, y)) {
            return moisture;
        }

        // 如果还是没有，单独计算湿度
        self.calculate_moisture(x, y)
    }

    /// 计算湿度，内部函数
    fn calculate_moisture(&self, x: i32, y: i32) -> f32 {
        // 计算基础湿度
        let nx = x as f64 * 0.015;
        let ny = y as f64 * 0.015;
        let base_moisture = self.moisture_noise.get([nx, ny]) as f32;

        // 纬度影响（低纬度地区通常更湿润）
        let world_height = 10000.0; // 假设世界总高度
        let latitude_factor = (y as f32 / world_height).min(1.0);
        let latitude_effect = (1.0 - latitude_factor) * self.params.latitude_moisture_factor;

        // 季节影响
        let season_effect = match self.current_season {
            Season::Summer => -0.1, // 夏季蒸发更多
            Season::Spring => 0.2,  // 春季更湿润
            Season::Autumn => 0.1,  // 秋季稍湿润
            Season::Winter => -0.2, // 冬季更干燥
        };

        // 距水源距离影响（简化模拟）
        let water_nx = x as f64 * 0.005;
        let water_ny = y as f64 * 0.005;
        let water_proximity = (self.moisture_noise.get([water_nx, water_ny]) + 1.0) * 0.5;
        let water_effect = water_proximity as f32 * 0.3;

        // 生成最终湿度
        let moisture = ((base_moisture + 1.0) * 0.5 // 转换到0-1
            + latitude_effect
            + season_effect
            + water_effect)
            * self.params.moisture_scale
            + self.params.moisture_offset;

        // 标准化到0.0-1.0范围
        moisture.min(1.0).max(0.0)
    }

    /// 获取指定位置的气候区域类型
    pub fn get_climate_zone(&self, x: i32, y: i32, height: f32) -> Zone {
        let temperature = self.get_temperature(x, y);
        let moisture = self.get_moisture(x, y);

        // 根据高度确定是否为山地
        if height > 0.7 {
            return Zone::Mountains;
        }

        // 根据温度和湿度确定气候区域
        match (temperature, moisture) {
            // 极地 - 寒冷
            (t, _) if t < 0.2 => Zone::Polar,

            // 沙漠 - 炎热干燥
            (t, m) if t > 0.7 && m < 0.3 => Zone::Desert,

            // 热带 - 温暖潮湿
            (t, m) if t > 0.6 && m > 0.5 => Zone::Tropical,

            // 大陆性 - 温度变化大，较干燥
            (t, m) if t > 0.3 && t < 0.7 && m < 0.5 => Zone::Continental,

            // 温带 - 温和适中
            _ => Zone::Temperate,
        }
    }
}
