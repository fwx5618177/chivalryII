use bevy::prelude::*;

use super::{area::TerrainConfig, Climate, Vegetation, Water};

/// 地图管理器
/// 负责管理地图的核心组件和规则
#[derive(Resource)]
pub struct MapManager {
    /// 地图种子
    pub seed: u32,
    /// 地形配置
    pub terrain_config: TerrainConfig,
    /// 水系配置
    pub water_config: Water,
    /// 植被配置
    pub vegetation_config: Vegetation,
    /// 气候配置
    pub climate_config: Climate,
    /// 高度缩放因子
    pub height_scale: f32,
    /// 是否启用2.5D效果
    pub enable_2_5d: bool,
}

impl Default for MapManager {
    fn default() -> Self {
        Self {
            seed: 42,
            terrain_config: TerrainConfig::default(),
            water_config: Water::default(),
            vegetation_config: Vegetation::default(),
            climate_config: Climate::default(),
            height_scale: 0.5,
            enable_2_5d: true,
        }
    }
}

impl MapManager {
    /// 创建新的地图管理器
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }

    /// 获取指定位置的高度值
    pub fn get_height_at(&self, _x: i32, _y: i32) -> f32 {
        // 这里只提供接口，实际实现由Chunk模块负责
        // 返回一个默认值，实际应用中会被覆盖
        0.0
    }

    /// 设置高度缩放因子
    pub fn set_height_scale(&mut self, scale: f32) {
        self.height_scale = scale;
    }

    /// 设置是否启用2.5D效果
    pub fn set_enable_2_5d(&mut self, enable: bool) {
        self.enable_2_5d = enable;
    }

    /// 获取地形配置
    pub fn terrain_config(&self) -> &TerrainConfig {
        &self.terrain_config
    }

    /// 获取水系配置
    pub fn water_config(&self) -> &Water {
        &self.water_config
    }

    /// 获取植被配置
    pub fn vegetation_config(&self) -> &Vegetation {
        &self.vegetation_config
    }

    /// 获取气候配置
    pub fn climate_config(&self) -> &Climate {
        &self.climate_config
    }

    /// 更新地形配置
    pub fn update_terrain_config(&mut self, config: TerrainConfig) {
        self.terrain_config = config;
    }

    /// 更新水系配置
    pub fn update_water_config(&mut self, config: Water) {
        self.water_config = config;
    }

    /// 更新植被配置
    pub fn update_vegetation_config(&mut self, config: Vegetation) {
        self.vegetation_config = config;
    }

    /// 更新气候配置
    pub fn update_climate_config(&mut self, config: Climate) {
        self.climate_config = config;
    }
}
