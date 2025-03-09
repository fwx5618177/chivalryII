use crate::world::map::EnvironmentParams;

use super::super::{
    tile::{Render as TileRender, TileType},
    vegetation::Rule as VegetationRules,
    WaterManager,
};
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

/// 地形兼容性规则
#[derive(Debug, Clone)]
pub struct TerrainCompatibility {
    /// 理想高度范围
    pub ideal_height: (f32, f32),
    /// 可接受高度范围
    pub acceptable_height: (f32, f32),
    /// 理想温度范围
    pub ideal_temperature: (f32, f32),
    /// 可接受温度范围
    pub acceptable_temperature: (f32, f32),
    /// 理想湿度范围
    pub ideal_moisture: (f32, f32),
    /// 可接受湿度范围
    pub acceptable_moisture: (f32, f32),
}

impl Default for TerrainCompatibility {
    fn default() -> Self {
        Self {
            ideal_height: (0.4, 0.6),
            acceptable_height: (0.3, 0.7),
            ideal_temperature: (0.4, 0.6),
            acceptable_temperature: (0.3, 0.7),
            ideal_moisture: (0.4, 0.6),
            acceptable_moisture: (0.3, 0.7),
        }
    }
}

impl TerrainCompatibility {
    /// 检查地形是否符合要求
    pub fn check_compatibility(&self, env: &EnvironmentParams) -> bool {
        false
    }
}

/// 地形配置
///
/// 定义地形生成的规则和参数
#[derive(Debug, Clone)]
pub struct TerrainConfig {
    /// 噪声振幅
    pub amplitude: f32,
    /// 噪声频率
    pub frequency: f64,
    /// 噪声层数
    pub octaves: usize,
    /// 噪声持续度
    pub persistence: f32,
    /// 噪声粗糙度
    pub lacunarity: f64,
    /// 高度缩放
    pub height_scale: f32,
    /// 高度偏移
    pub height_offset: f32,
    /// 水面高度
    pub water_level: f32,

    // 山脉特征参数
    /// 是否启用山脉
    pub enable_mountains: bool,
    /// 山脉频率
    pub mountain_frequency: f64,
    /// 山脉高度
    pub mountain_height: f32,
    /// 山脉阈值
    pub mountain_threshold: f32,

    // 平原特征参数
    /// 是否启用平原
    pub enable_plains: bool,
    /// 平原频率
    pub plain_frequency: f64,
    /// 平原高度
    pub plain_height: f32,
    /// 平原阈值
    pub plain_threshold: f32,
    /// 平原强度
    pub plain_strength: f32,

    // 河流特征参数
    /// 是否启用河流
    pub enable_rivers: bool,
    /// 河流频率
    pub river_frequency: f64,
    /// 河流宽度
    pub river_width: f32,
    /// 河流深度
    pub river_depth: f32,

    // 生物群系参数
    /// 生物群系频率
    pub biome_frequency: f64,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            frequency: 0.01,
            octaves: 4,
            persistence: 0.5,
            lacunarity: 2.0,
            height_scale: 1.0,
            height_offset: 0.0,
            water_level: 0.3,

            enable_mountains: true,
            mountain_frequency: 0.005,
            mountain_height: 0.5,
            mountain_threshold: 0.6,

            enable_plains: true,
            plain_frequency: 0.008,
            plain_height: 0.2,
            plain_threshold: 0.5,
            plain_strength: 0.7,

            enable_rivers: true,
            river_frequency: 0.01,
            river_width: 0.05,
            river_depth: 0.2,

            biome_frequency: 0.02,
        }
    }
}

impl TerrainConfig {
    /// 创建新的地形配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建山地地形配置
    pub fn mountain() -> Self {
        Self {
            mountain_height: 0.8,
            mountain_threshold: 0.4,
            mountain_frequency: 0.008,
            water_level: 0.25,
            ..Default::default()
        }
    }

    /// 创建平原地形配置
    pub fn plains() -> Self {
        Self {
            plain_strength: 0.9,
            plain_threshold: 0.3,
            mountain_threshold: 0.8,
            water_level: 0.2,
            ..Default::default()
        }
    }

    /// 创建河谷地形配置
    pub fn river_valley() -> Self {
        Self {
            river_width: 0.1,
            river_depth: 0.3,
            river_frequency: 0.015,
            water_level: 0.35,
            ..Default::default()
        }
    }
}

/// 地形生成器实现
#[derive(Debug)]
pub struct TerrainGenerator {
    /// 噪声生成器
    noise: Perlin,
    /// 地形配置
    config: TerrainConfig,
}

impl Default for TerrainGenerator {
    fn default() -> Self {
        Self {
            noise: Perlin::new(42),
            config: TerrainConfig::default(),
        }
    }
}

impl TerrainGenerator {
    /// 创建新的地形生成器
    pub fn new(seed: u32, config: TerrainConfig) -> Self {
        let noise = Perlin::new(seed);
        Self { noise, config }
    }

    pub fn initialize(&mut self, seed: u32) {
        self.noise = Perlin::new(seed);
    }

    /// 生成指定位置的高度值
    pub fn generate_height(&self, x: f64, y: f64) -> f32 {
        let mut height = 0.0;

        // 多层噪声叠加
        let mut amplitude = self.config.amplitude;
        let mut frequency = self.config.frequency;

        for _ in 0..self.config.octaves {
            let nx = x * frequency;
            let ny = y * frequency;

            // 使用噪声函数生成值
            let noise_val = self.noise.get([nx, ny]) as f32;
            height += noise_val * amplitude;

            // 调整下一层的振幅和频率
            amplitude *= self.config.persistence;
            frequency *= self.config.lacunarity;
        }

        // 应用高度缩放和偏移
        height = height * self.config.height_scale + self.config.height_offset;

        // 应用地形特征
        self.apply_terrain_features(x, y, height)
    }

    /// 应用地形特征（如山脉、平原等）
    fn apply_terrain_features(&self, x: f64, y: f64, base_height: f32) -> f32 {
        let mut height = base_height;

        // 山脉特征
        if self.config.enable_mountains {
            let mountain_noise = self.noise.get([
                x * self.config.mountain_frequency,
                y * self.config.mountain_frequency,
            ]) as f32;

            if mountain_noise > self.config.mountain_threshold {
                let mountain_factor = (mountain_noise - self.config.mountain_threshold)
                    / (1.0 - self.config.mountain_threshold);
                height += self.config.mountain_height * mountain_factor * mountain_factor;
            }
        }

        // 平原特征
        if self.config.enable_plains {
            let plain_noise = self.noise.get([
                x * self.config.plain_frequency + 1000.0,
                y * self.config.plain_frequency + 1000.0,
            ]) as f32;

            if plain_noise > self.config.plain_threshold {
                let plain_factor = (plain_noise - self.config.plain_threshold)
                    / (1.0 - self.config.plain_threshold);
                height = height * (1.0 - plain_factor * self.config.plain_strength)
                    + self.config.plain_height * plain_factor * self.config.plain_strength;
            }
        }

        // 河流特征
        if self.config.enable_rivers {
            let river_noise = self.noise.get([
                x * self.config.river_frequency + 2000.0,
                y * self.config.river_frequency + 2000.0,
            ]) as f32;

            if river_noise.abs() < self.config.river_width {
                let river_factor = 1.0 - (river_noise.abs() / self.config.river_width);
                height -= self.config.river_depth * river_factor * river_factor;
            }
        }

        height
    }

    /// 根据高度和其他因素确定瓦片类型
    pub fn determine_tile_type(&self, height: f32, x: f64, y: f64) -> u8 {
        // 水面高度阈值
        let water_level = self.config.water_level;

        // 基于高度的基本类型判断
        let base_type = if height < water_level {
            TileType::Water as u8
        } else if height < water_level + 0.05 {
            TileType::Sand as u8
        } else if height < water_level + 0.3 {
            TileType::Grass as u8
        } else if height < water_level + 0.6 {
            TileType::Forest as u8
        } else if height < water_level + 0.8 {
            TileType::Mountain as u8
        } else {
            TileType::Snow as u8
        };

        // 应用生物群系变化
        self.apply_biome_variations(base_type, height, x, y)
    }

    /// 应用生物群系变化
    fn apply_biome_variations(&self, base_type: u8, height: f32, x: f64, y: f64) -> u8 {
        // 使用额外的噪声来确定生物群系变化
        let biome_noise = self.noise.get([
            x * self.config.biome_frequency + 3000.0,
            y * self.config.biome_frequency + 3000.0,
        ]) as f32;

        match base_type {
            // 草地可能变成平原或荒地
            t if t == TileType::Grass as u8 => {
                if biome_noise > 0.6 {
                    TileType::Plains as u8
                } else if biome_noise < -0.6 {
                    TileType::Wasteland as u8
                } else {
                    base_type
                }
            }

            // 森林可能变成竹林或密林
            t if t == TileType::Forest as u8 => {
                if biome_noise > 0.7 {
                    TileType::Bamboo as u8
                } else if biome_noise < -0.7 {
                    TileType::DenseForest as u8
                } else {
                    base_type
                }
            }

            // 其他类型保持不变
            _ => base_type,
        }
    }

    pub fn get_height(&self, x: f64, y: f64) -> f32 {
        self.generate_height(x, y)
    }

    pub fn get_slope(&self, x: f64, y: f64) -> f32 {
        let dx = 0.01;
        let dy = 0.01;

        let center = self.generate_height(x, y);
        let north = self.generate_height(x, y + dy);
        let south = self.generate_height(x, y - dy);
        let east = self.generate_height(x + dx, y);
        let west = self.generate_height(x - dx, y);

        let dz_dx = (east - west) / (2.0 * dx as f32);
        let dz_dy = (north - south) / (2.0 * dy as f32);

        (dz_dx * dz_dx + dz_dy * dz_dy).sqrt()
    }
}

/// 地形渲染辅助函数
pub mod terrain_render {
    use super::*;

    /// 根据高度和瓦片类型生成颜色
    pub fn generate_terrain_color(height: f32, tile_type: TileType) -> Color {
        let tile_render = TileRender::from_tile_type(tile_type);
        let base_color = tile_render.color.to_srgba();

        // 根据高度调整颜色
        let height_factor = (height - 0.2).clamp(0.0, 0.8) / 0.8;
        let shadow_factor = 1.0 - height_factor * 0.3;

        Color::rgb(
            base_color.red * shadow_factor,
            base_color.green * shadow_factor,
            base_color.blue * shadow_factor,
        )
    }

    /// 生成地形边缘效果
    pub fn generate_edge_effect(height: f32, neighbor_height: f32) -> Color {
        let height_diff = (height - neighbor_height).abs();

        if height_diff > 0.1 {
            // 边缘效果强度
            let edge_strength = (height_diff - 0.1).clamp(0.0, 0.3) / 0.3;

            // 如果当前高度大于邻居高度，生成阴影效果
            if height > neighbor_height {
                Color::rgba(0.0, 0.0, 0.0, edge_strength * 0.5)
            }
            // 否则生成高光效果
            else {
                Color::rgba(1.0, 1.0, 1.0, edge_strength * 0.3)
            }
        } else {
            Color::rgba(0.0, 0.0, 0.0, 0.0) // 透明色，无效果
        }
    }
}

/// 地形高度类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainHeight {
    Valley,   // 山谷(0-0.2)
    Plain,    // 平原(0.2-0.4)
    Hill,     // 丘陵(0.4-0.6)
    Mountain, // 山地(0.6-0.8)
    Peak,     // 山峰(0.8-1.0)
}

/// 地形参数配置
#[derive(Debug, Clone)]
pub struct TerrainParams {
    /// 基础高度系数
    base_height_scale: f32,

    /// 山脉分布密度
    mountain_density: f32,

    /// 水系分布规则
    water_distribution: WaterManager,

    /// 植被分布规则
    vegetation_rules: VegetationRules,
}

impl TerrainParams {
    /// 创建武侠风格的默认地形参数
    pub fn wuxia_default() -> Self {
        Self {
            base_height_scale: 1.2, // 较高的地形起伏
            mountain_density: 0.3,  // 适中的山脉密度
            water_distribution: WaterManager::wuxia_style(),
            vegetation_rules: VegetationRules::default(),
        }
    }
}
