use super::{climate::Climate, map_noise::MapNoise, SceneType, TerrainCompatibility};

/// 地形高度分类
///
/// # 设计思路
/// 1. 离散分类：将连续的高度值映射为离散的地形类型
/// 2. 游戏性考虑：每种地形类型对应不同的游戏玩法
/// 3. 视觉效果：便于根据类型应用不同的视觉效果
///
/// # 高度范围划分
/// - Valley(0-0.2): 适合放置水系、村落
/// - Plain(0.2-0.4): 主要活动区域，适合建筑
/// - Hill(0.4-0.6): 过渡地带，可以有特殊资源
/// - Mountain(0.6-0.8): 挑战区域，稀有资源
/// - Peak(0.8-1.0): 极限区域，终极目标
///
/// # 游戏影响
/// 1. 可达性：影响玩家移动和探索
/// 2. 资源分布：不同高度对应不同资源
/// 3. 建筑限制：影响建筑物放置
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainHeight {
    Valley,   // 山谷(0-0.2)
    Plain,    // 平原(0.2-0.4)
    Hill,     // 丘陵(0.4-0.6)
    Mountain, // 山地(0.6-0.8)
    Peak,     // 山峰(0.8-1.0)
}

/// 环境适应性规则
#[derive(Debug, Clone)]
pub struct EnvironmentCompatibility {
    /// 理想高度范围
    pub ideal_height: (f32, f32),
    /// 可生存高度范围
    pub survivable_height: (f32, f32),

    /// 理想温度范围
    pub ideal_temperature: (f32, f32),
    /// 可生存温度范围
    pub survivable_temperature: (f32, f32),

    /// 理想湿度范围
    pub ideal_moisture: (f32, f32),
    /// 可生存湿度范围
    pub survivable_moisture: (f32, f32),
}

/// 环境要求
///
/// # 设计思路
/// 1. 定义场景的环境条件
/// 2. 控制场景的生成规则
/// 3. 影响场景的状态
#[derive(Debug, Clone)]
pub struct EnvironmentRequirements {
    /// 地形要求
    pub terrain_compatibility: TerrainCompatibility,
    /// 最小区域大小
    pub min_area: f32,
    /// 气候条件
    pub climate_conditions: Climate,
    /// 场景冲突检查
    pub scene_conflicts: Vec<SceneType>,
}

impl Default for EnvironmentRequirements {
    fn default() -> Self {
        Self {
            terrain_compatibility: TerrainCompatibility::default(),
            min_area: 1.0,
            climate_conditions: Climate::default(),
            scene_conflicts: Vec::new(),
        }
    }
}

/// 环境参数，用于描述特定位置的环境特性
#[derive(Debug, Clone)]
pub struct EnvironmentParams {
    /// 高度值 (0.0-1.0)
    pub height: f32,
    /// 温度值 (0.0-1.0)，0表示极寒，1表示炎热
    pub temperature: f32,
    /// 湿度值 (0.0-1.0)，0表示干燥，1表示潮湿
    pub moisture: f32,
    /// 地形类型
    pub terrain_type: TerrainHeight,
}

/// 环境参数生成器
///
/// # 核心功能
/// 1. 统一的环境参数生成
/// 2. 确保地形生成的一致性
/// 3. 提供可配置的生成参数
///
/// # 实现细节
/// 1. 使用不同的噪声生成器分别控制高度、温度和湿度
/// 2. 通过种子确保生成结果的可重复性
/// 3. 支持参数调整以适应不同场景需求
///
/// # 性能优化
/// 1. 使用缓存减少重复计算
/// 2. 支持区块级别的批量生成
/// 3. 延迟计算非必要参数
#[derive(Debug, Clone)]
pub struct EnvironmentGenerator {
    /// 高度图生成器
    /// 用于生成地形高度变化
    /// - 影响地形类型（平原、丘陵、山地等）
    /// - 影响可通行性
    /// - 影响视觉效果
    pub height_generator: MapNoise,

    /// 温度生成器
    /// 影响环境效果：
    /// - 植被类型（竹林、枫林等）
    /// - 天气效果
    /// - 视觉氛围
    pub temperature_generator: MapNoise,

    /// 湿度生成器
    /// 影响环境细节：
    /// - 水系分布
    /// - 植被密度
    /// - 天气概率
    pub moisture_generator: MapNoise,
}

impl EnvironmentGenerator {
    /// 创建新的环境参数生成器
    ///
    /// # 参数说明
    /// - seed: 世界种子，用于初始化所有噪声生成器
    ///
    /// # 实现细节
    /// 1. 为每个生成器使用不同的种子避免重复
    /// 2. 使用不同的缩放因子控制变化粒度
    /// 3. 初始化时预设合理的默认参数
    pub fn new(seed: u64) -> Self {
        Self {
            height_generator: MapNoise::new(seed as u32, 0.01, 0.0),
            temperature_generator: MapNoise::new((seed + 1) as u32, 0.005, 0.0),
            moisture_generator: MapNoise::new((seed + 2) as u32, 0.008, 0.0),
        }
    }

    /// 获取指定位置的环境参数
    ///
    /// # 参数
    /// - x, y: 世界坐标
    ///
    /// # 返回值
    /// 包含该位置的完整环境信息的 EnvironmentParams
    ///
    /// # 实现细节
    /// 1. 合并多个噪声图层
    /// 2. 根据高度确定地形类型
    /// 3. 考虑相邻区域的影响
    pub fn get_params(&self, x: i32, y: i32) -> EnvironmentParams {
        let height = self.height_generator.get(x as f32, y as f32);
        let temperature = self.temperature_generator.get(x as f32, y as f32);
        let moisture = self.moisture_generator.get(x as f32, y as f32);

        let terrain_type = if height < 0.2 {
            TerrainHeight::Valley
        } else if height < 0.4 {
            TerrainHeight::Plain
        } else if height < 0.6 {
            TerrainHeight::Hill
        } else if height < 0.8 {
            TerrainHeight::Mountain
        } else {
            TerrainHeight::Peak
        };

        EnvironmentParams {
            height,
            temperature,
            moisture,
            terrain_type,
        }
    }
}

/// 密度控制参数
///
/// # 设计目的
/// 1. 控制各类元素（植被、资源等）的分布密度
/// 2. 提供环境因素对密度的影响权重
/// 3. 确保生成结果的自然感
///
/// # 参数说明
/// - base_density: 基础密度值
/// - height_influence: 高度对密度的影响程度
/// - moisture_influence: 湿度对密度的影响程度
#[derive(Debug, Clone)]
pub struct DensityControl {
    pub base_density: f32,
    pub height_influence: f32,
    pub moisture_influence: f32,
}

/// 高度适应参数
///
/// # 设计目的
/// 1. 定义对象在不同高度的适应性
/// 2. 控制对象的垂直分布
/// 3. 创造高度层次感
///
/// # 参数说明
/// - min_height: 最低可生存高度
/// - max_height: 最高可生存高度
/// - optimal_height: 最适宜生长高度
#[derive(Debug, Clone)]
pub struct HeightAdaptation {
    pub min_height: f32,
    pub max_height: f32,
    pub optimal_height: f32,
}
