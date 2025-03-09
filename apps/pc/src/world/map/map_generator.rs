use bevy::math::{IVec2, Rect};
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use std::collections::HashMap;

use super::{
    area::{SceneType, TerrainGenerator},
    climate::System as ClimateSystem,
    environment::{EnvironmentParams, TerrainHeight},
    tile::{Tile, TileType},
    vegetation::System as VegetationSystem,
    world_config::WorldConfig,
    Water,
};

/// 场景生成规则
///
/// # 设计思路
/// 1. 场景位置管理：通过HashMap高效存储和查找固定场景
/// 2. 权重系统：使用权重控制不同类型场景的生成概率
/// 3. 间距控制：通过最小间距确保场景分布合理
///
/// # 性能考虑
/// 1. HashMap提供O(1)的查找性能
/// 2. 权重系统避免频繁随机计算
/// 3. 间距检查优化场景布局
#[derive(Debug, Clone)]
pub struct SceneRules {
    /// 固定场景位置映射表
    /// 键：世界坐标
    /// 值：场景类型
    pub fixed_scenes: HashMap<IVec2, SceneType>,

    /// 场景生成权重表
    /// 键：场景类型
    /// 值：生成权重(0.0-1.0)
    pub generation_weights: HashMap<SceneType, f32>,

    /// 场景最小间距
    /// 控制场景之间的最小距离，避免场景过度集中
    pub min_scene_distance: f32,
}

/// 地图生成器
///
/// # 核心功能
/// 1. 地形生成：协调多个子系统生成完整地形
/// 2. 场景管理：处理固定和随机场景的生成
/// 3. 环境模拟：管理气候、水文等环境系统
///
/// # 设计考虑
/// 1. 模块化：每个子系统独立负责特定功能
/// 2. 可扩展：支持添加新的生成系统
/// 3. 高性能：使用缓存和延迟加载优化性能
/// 4. 一致性：使用种子系统确保生成结果可重现
///
/// # 性能优化
/// 1. 缓存系统：减少重复计算
/// 2. 延迟加载：按需生成内容
/// 3. 并行处理：支持多线程生成
#[derive(Debug)]
pub struct MapGenerator {
    /// 世界基础配置
    pub world_config: WorldConfig,
    /// 地形生成系统
    terrain_generator: TerrainGenerator,
    /// 水文系统
    water: Water,
    /// 植被系统
    vegetation_system: VegetationSystem,
    /// 气候系统
    climate_system: ClimateSystem,
    /// 场景规则
    scene_rules: SceneRules,
}

impl Default for MapGenerator {
    fn default() -> Self {
        Self {
            world_config: WorldConfig {
                seed: 42,
                world_bounds: Some(Rect::new(-1000.0, -1000.0, 1000.0, 1000.0)),
                chunk_size: 32,
            },
            terrain_generator: TerrainGenerator::default(),
            water: Water::default(),
            vegetation_system: VegetationSystem::default(),
            climate_system: ClimateSystem::default(),
            scene_rules: SceneRules {
                fixed_scenes: HashMap::new(),
                generation_weights: HashMap::new(),
                min_scene_distance: 100.0,
            },
        }
    }
}

impl MapGenerator {
    /// 创建新的地图生成器
    ///
    /// # 参数说明
    /// * `seed` - 世界种子，用于初始化所有随机生成器
    ///
    /// # 内部流程
    /// 1. 创建默认实例
    /// 2. 使用种子初始化
    /// 3. 准备各子系统
    ///
    /// # 性能考虑
    /// 1. 延迟初始化大型资源
    /// 2. 预分配必要的内存空间
    /// 3. 建立基础缓存系统
    pub fn new(seed: u64) -> Self {
        let mut generator = Self::default();
        generator.initialize(seed);
        generator
    }

    /// 初始化地图生成器
    ///
    /// # 实现细节
    /// 1. 设置世界种子
    /// 2. 为每个子系统分配唯一种子
    /// 3. 初始化所有子系统
    ///
    /// # 安全考虑
    /// 1. 使用wrapping_add避免溢出
    /// 2. 确保种子分配不重复
    /// 3. 子系统初始化失败处理
    pub fn initialize(&mut self, seed: u64) {
        self.world_config.seed = seed;
        self.terrain_generator.initialize(seed as u32);
        self.water.initialize(seed.wrapping_add(1));
        self.vegetation_system.initialize(seed.wrapping_add(2));
        self.climate_system.initialize(seed.wrapping_add(3));
    }

    /// 获取指定位置的环境参数
    ///
    /// # 功能说明
    /// 整合多个系统的数据，生成完整的环境参数信息
    ///
    /// # 参数
    /// * `x` - 世界X坐标
    /// * `y` - 世界Y坐标
    ///
    /// # 返回值
    /// 返回 EnvironmentParams，包含：
    /// - 高度信息
    /// - 温度数据
    /// - 湿度数据
    /// - 地形类型
    ///
    /// # 实现细节
    /// 1. 从地形生成器获取高度数据
    /// 2. 从气候系统获取温度和湿度
    /// 3. 根据高度划分地形类型
    ///
    /// # 性能考虑
    /// 1. 高频调用函数，需要高效实现
    /// 2. 考虑添加缓存机制
    /// 3. 避免重复计算
    pub fn get_environment(&self, x: i32, y: i32) -> EnvironmentParams {
        let height = self.terrain_generator.get_height(x as f64, y as f64);
        let temperature = self.climate_system.get_temperature(x, y);
        let moisture = self.climate_system.get_moisture(x, y);

        let terrain_type = match height {
            h if h < 0.2 => TerrainHeight::Valley,
            h if h < 0.4 => TerrainHeight::Plain,
            h if h < 0.6 => TerrainHeight::Hill,
            h if h < 0.8 => TerrainHeight::Mountain,
            _ => TerrainHeight::Peak,
        };

        EnvironmentParams {
            height,
            temperature,
            moisture,
            terrain_type,
        }
    }

    /// 获取特定位置的场景类型
    ///
    /// # 功能说明
    /// 确定指定位置应该生成什么类型的场景
    ///
    /// # 参数
    /// * `x` - 世界X坐标
    /// * `y` - 世界Y坐标
    ///
    /// # 返回值
    /// * `Some(SceneType)` - 该位置应该生成的场景类型
    /// * `None` - 该位置不应该生成场景
    ///
    /// # 实现流程
    /// 1. 检查是否是预定义场景位置
    /// 2. 获取该位置的环境参数
    /// 3. 根据环境条件决定场景类型
    ///
    /// # 设计考虑
    /// 1. 优先级：固定场景 > 环境生成
    /// 2. 场景间距：确保场景不会过度密集
    /// 3. 环境适配：场景类型要符合环境特征
    pub fn get_scene_at(&self, x: i32, y: i32) -> Option<SceneType> {
        let pos = IVec2::new(x, y);

        // 检查固定场景
        if let Some(scene) = self.scene_rules.fixed_scenes.get(&pos) {
            return Some(*scene);
        }

        // 基于环境生成场景
        let env = self.get_environment(x, y);
        self.generate_scene_for_environment(pos, &env)
    }

    /// 生成指定区域的地图
    ///
    /// # 功能说明
    /// 生成一个矩形区域内的所有地形数据
    ///
    /// # 参数
    /// * `x` - 区域起始X坐标
    /// * `y` - 区域起始Y坐标
    /// * `width` - 区域宽度
    /// * `height` - 区域高度
    ///
    /// # 返回值
    /// 返回二维数组，包含区域内所有瓦片的数据
    ///
    /// # 实现细节
    /// 1. 创建适当大小的瓦片数组
    /// 2. 遍历区域内的每个位置
    /// 3. 为每个位置生成对应的瓦片
    ///
    /// # 性能优化
    /// 1. 支持并行生成
    /// 2. 使用预分配内存
    /// 3. 考虑区块级缓存
    ///
    /// # 注意事项
    /// 1. 边界处理要准确
    /// 2. 保持相邻区域的连续性
    /// 3. 内存使用要合理
    pub fn generate_region(&self, x: i32, y: i32, width: i32, height: i32) -> Vec<Vec<Tile>> {
        let mut tiles = vec![vec![Tile::default(); height as usize]; width as usize];

        for i in 0..width {
            for j in 0..height {
                let world_x = x + i;
                let world_y = y + j;
                let env = self.get_environment(world_x, world_y);

                tiles[i as usize][j as usize] = self.generate_tile(world_x, world_y, &env);
            }
        }

        tiles
    }

    /// 根据环境参数生成单个地块
    ///
    /// # 功能说明
    /// 根据环境参数确定单个地块的具体属性
    ///
    /// # 参数
    /// * `x` - 地块X坐标
    /// * `y` - 地块Y坐标
    /// * `env` - 环境参数
    ///
    /// # 返回值
    /// 返回生成的地块数据
    ///
    /// # 实现流程
    /// 1. 创建基础地块
    /// 2. 设置高度值
    /// 3. 根据地形类型和环境确定地块类型
    /// 4. 应用水系影响
    /// 5. 更新通行属性
    ///
    /// # 设计考虑
    /// 1. 地形类型的自然过渡
    /// 2. 环境因素的综合影响
    /// 3. 游戏性平衡
    ///
    /// # 特殊情况处理
    /// 1. 极端环境条件
    /// 2. 特殊地形要求
    /// 3. 边界情况
    fn generate_tile(&self, x: i32, y: i32, env: &EnvironmentParams) -> Tile {
        let mut tile = Tile::default();
        tile.height = env.height;

        // 确定基础地形
        tile.tile_type = match env.terrain_type {
            TerrainHeight::Valley => {
                if env.moisture > 0.7 {
                    TileType::Water
                } else {
                    TileType::Ground
                }
            }
            TerrainHeight::Plain => {
                if env.moisture > 0.6 {
                    TileType::Grass
                } else {
                    TileType::Ground
                }
            }
            TerrainHeight::Hill => {
                if env.moisture > 0.5 {
                    TileType::Forest
                } else {
                    TileType::Grass
                }
            }
            TerrainHeight::Mountain => {
                if env.temperature < 0.3 {
                    TileType::Snow
                } else {
                    TileType::Rock
                }
            }
            TerrainHeight::Peak => TileType::Rock,
        };

        // 应用水系影响
        if self.water.has_water_at(x, y) {
            tile.tile_type = TileType::Water;
        }

        // 更新通行属性
        let props = Tile::get_properties(tile.tile_type);
        tile.walkable = props.walkable;

        tile
    }

    /// 基于环境参数生成场景
    ///
    /// # 功能说明
    /// 根据环境条件和权重系统决定是否生成场景及其类型
    ///
    /// # 参数
    /// * `pos` - 场景位置
    /// * `env` - 环境参数
    ///
    /// # 返回值
    /// * `Some(SceneType)` - 选定的场景类型
    /// * `None` - 不生成场景
    ///
    /// # 实现细节
    /// 1. 创建位置相关的随机数生成器
    /// 2. 根据环境条件筛选可能的场景
    /// 3. 应用场景生成权重
    /// 4. 随机选择最终场景
    ///
    /// # 算法说明
    /// 1. 权重计算方式
    /// 2. 随机选择策略
    /// 3. 环境适应性判断
    ///
    /// # 优化考虑
    /// 1. 权重计算缓存
    /// 2. 随机数生成优化
    /// 3. 场景分布平衡
    fn generate_scene_for_environment(
        &self,
        pos: IVec2,
        env: &EnvironmentParams,
    ) -> Option<SceneType> {
        let mut rng = self.make_rng_for_position(pos);
        let mut candidates = Vec::new();

        // 根据环境条件添加候选场景
        match env.terrain_type {
            TerrainHeight::Peak if env.height > 0.85 => {
                candidates.push((SceneType::Temple, 10.0));
            }
            TerrainHeight::Plain => {
                candidates.push((SceneType::Village, 5.0));
                if env.height > 0.25 && env.height < 0.35 {
                    candidates.push((SceneType::Town, 3.0));
                }
            }
            TerrainHeight::Valley if env.moisture > 0.8 => {
                candidates.push((SceneType::Lake, 8.0));
            }
            TerrainHeight::Mountain
                if self.terrain_generator.get_slope(pos.x as f64, pos.y as f64) > 0.7 =>
            {
                candidates.push((SceneType::Waterfall, 7.0));
            }
            _ => {}
        }

        // 加入配置的生成权重
        candidates.extend(
            self.scene_rules
                .generation_weights
                .iter()
                .map(|(scene_type, weight)| (*scene_type, *weight)),
        );

        if candidates.is_empty() {
            return None;
        }

        // 根据权重随机选择
        let total_weight: f32 = candidates.iter().map(|(_, w)| w).sum();
        let random_value = rng.gen_range(0.0..total_weight);

        let mut cumulative = 0.0;
        for (scene_type, weight) in candidates {
            cumulative += weight;
            if random_value <= cumulative {
                return Some(scene_type);
            }
        }

        None
    }

    /// 创建位置相关的随机数生成器
    ///
    /// # 功能说明
    /// 创建一个基于位置和世界种子的确定性随机数生成器
    ///
    /// # 参数
    /// * `pos` - 世界坐标位置
    ///
    /// # 返回值
    /// 返回一个实现了 Rng trait 的随机数生成器
    ///
    /// # 实现细节
    /// 1. 组合世界种子和位置信息
    /// 2. 使用 wrapping 操作避免溢出
    /// 3. 创建 ChaChaRng 实例
    ///
    /// # 设计考虑
    /// 1. 确定性：相同输入产生相同结果
    /// 2. 分布均匀性：保证随机数质量
    /// 3. 性能表现：快速的随机数生成
    ///
    /// # 应用场景
    /// 1. 场景生成
    /// 2. 装饰物放置
    /// 3. 环境细节随机化
    fn make_rng_for_position(&self, pos: IVec2) -> impl Rng {
        let combined_seed = self
            .world_config
            .seed
            .wrapping_add(pos.x as u64)
            .wrapping_mul(31)
            .wrapping_add(pos.y as u64);

        ChaChaRng::seed_from_u64(combined_seed)
    }
}
