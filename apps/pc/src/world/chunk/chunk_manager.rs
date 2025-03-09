use super::render::RenderSettings;
use crate::world::map::{MapManager, TerrainGenerator};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::CHUNK_SIZE;

/// 区块坐标系统
/// 使用整数坐标系统的原因：
/// 1. 精确定位：避免浮点数精度问题
/// 2. 哈希友好：整数坐标便于用作哈希表键
/// 3. 性能优化：整数运算比浮点运算更快
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

/// 区块加载状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkLoadState {
    /// 未加载
    Unloaded,
    /// 正在加载
    Loading,
    /// 已加载
    Loaded,
    /// 正在卸载
    Unloading,
}

/// 区块数据
#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct ChunkData {
    /// 瓦片类型数据
    tiles: Vec<Option<u8>>,
    /// 高度数据
    heights: Vec<f32>,
    /// 装饰物数据
    decorations: Vec<Option<u8>>,
    /// 是否被修改过
    pub modified: bool,
}

impl ChunkData {
    /// 创建新的区块数据
    pub fn new() -> Self {
        let size = CHUNK_SIZE * CHUNK_SIZE;
        Self {
            tiles: vec![None; size],
            heights: vec![0.0; size],
            decorations: vec![None; size],
            modified: false,
        }
    }

    /// 获取瓦片类型
    pub fn get_tile(&self, x: usize, y: usize) -> Option<u8> {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            let index = y * CHUNK_SIZE + x;
            self.tiles[index]
        } else {
            None
        }
    }

    /// 设置瓦片类型
    pub fn set_tile(&mut self, x: usize, y: usize, tile_type: u8) {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            let index = y * CHUNK_SIZE + x;
            self.tiles[index] = Some(tile_type);
        }
    }

    /// 获取高度值
    pub fn get_height(&self, x: usize, y: usize) -> f32 {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            let index = y * CHUNK_SIZE + x;
            self.heights[index]
        } else {
            0.0
        }
    }

    /// 设置高度值
    pub fn set_height(&mut self, x: usize, y: usize, height: f32) {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            let index = y * CHUNK_SIZE + x;
            self.heights[index] = height;
        }
    }

    /// 添加装饰物
    pub fn add_decoration(&mut self, x: usize, y: usize, decoration_type: u8) {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            let index = y * CHUNK_SIZE + x;
            self.decorations[index] = Some(decoration_type);
        }
    }

    /// 获取装饰物类型
    pub fn get_decoration(&self, x: usize, y: usize) -> Option<u8> {
        if x < CHUNK_SIZE && y < CHUNK_SIZE {
            let index = y * CHUNK_SIZE + x;
            self.decorations[index]
        } else {
            None
        }
    }
}

impl Default for ChunkData {
    fn default() -> Self {
        Self::new()
    }
}

/// 区块数据结构
/// 设计考虑：
/// 1. 数据分离：将静态数据和动态状态分开存储
/// 2. 内存优化：使用Option包装大型数据
/// 3. 状态追踪：记录访问时间和优先级
#[derive(Debug, Component)]
pub struct Chunk {
    /// 区块坐标
    pub coord: ChunkCoord,
    /// 加载状态
    pub load_state: ChunkLoadState,
    /// 区块数据
    pub data: Option<ChunkData>,
    /// 实体ID
    pub entity: Option<Entity>,
    /// 最后访问时间
    pub last_accessed: f64,
    /// 加载优先级
    pub priority: i32,
}

/// 区块管理器
/// 核心设计原则：
/// 1. 中央管理：统一管理所有区块的生命周期
/// 2. 性能优化：通过预算系统控制资源使用
/// 3. 内存管理：自动清理不活跃区块
/// 4. 可配置性：关键参数可以根据需求调整
#[derive(Resource)]
pub struct ChunkManager {
    /// 区块映射表
    pub chunks: HashMap<ChunkCoord, Entity>,
    /// 地形生成器
    terrain_generator: Option<TerrainGenerator>,
    /// 渲染设置
    render_settings: RenderSettings,
    /// 视图距离（以区块为单位）
    pub view_distance: i32,
    /// 玩家当前区块坐标
    pub player_chunk: Option<ChunkCoord>,
    /// 上次清理时间
    pub last_cleanup: f64,
    /// 加载队列
    pub loading_queue: Vec<ChunkCoord>,
    /// 内存预算（最大区块数量）
    pub memory_budget: usize,
    /// 每帧加载预算
    pub load_budget: usize,
    /// 区块大小
    pub chunk_size: f32,
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self {
            chunks: HashMap::new(),
            terrain_generator: None,
            render_settings: RenderSettings::default(),
            view_distance: 5,
            player_chunk: None,
            last_cleanup: 0.0,
            loading_queue: Vec::new(),
            memory_budget: 100,
            load_budget: 2,
            chunk_size: CHUNK_SIZE as f32,
        }
    }
}

impl ChunkManager {
    /// 创建新的区块管理器
    pub fn new(view_distance: i32) -> Self {
        Self {
            view_distance,
            ..Default::default()
        }
    }

    /// 初始化地形生成器
    pub fn initialize_terrain_generator(&mut self, map_manager: &MapManager) {
        let terrain_config = map_manager.terrain_config().clone();
        self.terrain_generator = Some(TerrainGenerator::new(map_manager.seed, terrain_config));

        // 更新渲染设置
        self.render_settings.enable_2_5d = map_manager.enable_2_5d;
        self.render_settings.height_scale = map_manager.height_scale;
    }

    /// 更新玩家位置
    pub fn update_player_position(&mut self, world_x: f32, world_y: f32) {
        let chunk_x = (world_x / (CHUNK_SIZE as f32 * 32.0)).floor() as i32;
        let chunk_y = (world_y / (CHUNK_SIZE as f32 * 32.0)).floor() as i32;
        let new_chunk = ChunkCoord {
            x: chunk_x,
            y: chunk_y,
        };

        if self.player_chunk != Some(new_chunk) {
            self.player_chunk = Some(new_chunk);
        }
    }

    /// 获取需要加载的区块
    pub fn get_chunks_to_load(&self) -> Vec<ChunkCoord> {
        let mut to_load = Vec::new();

        if let Some(player_chunk) = self.player_chunk {
            for y in -self.view_distance..=self.view_distance {
                for x in -self.view_distance..=self.view_distance {
                    let coord = ChunkCoord {
                        x: player_chunk.x + x,
                        y: player_chunk.y + y,
                    };

                    // 检查区块是否已存在
                    if !self.chunks.contains_key(&coord) {
                        to_load.push(coord);
                    }
                }
            }
        }

        to_load
    }

    /// 获取需要卸载的区块
    pub fn get_chunks_to_unload(&self) -> Vec<ChunkCoord> {
        let mut to_unload = Vec::new();

        if let Some(player_chunk) = self.player_chunk {
            for (coord, _) in &self.chunks {
                let dx = (coord.x - player_chunk.x).abs();
                let dy = (coord.y - player_chunk.y).abs();

                // 如果区块超出视图距离，标记为卸载
                if dx > self.view_distance || dy > self.view_distance {
                    to_unload.push(*coord);
                }
            }
        }

        to_unload
    }

    /// 创建新区块
    pub fn create_chunk(&mut self, coord: ChunkCoord) -> Entity {
        let chunk_entity = Entity::from_raw(0); // Placeholder entity, will be replaced later
        self.chunks.insert(coord, chunk_entity);
        chunk_entity
    }

    /// 生成区块数据
    pub fn generate_chunk_data(&self, coord: ChunkCoord, map_manager: &MapManager) -> ChunkData {
        let mut data = ChunkData::new();

        if let Some(generator) = &self.terrain_generator {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    // 计算世界坐标
                    let world_x = coord.x * CHUNK_SIZE as i32 + x as i32;
                    let world_y = coord.y * CHUNK_SIZE as i32 + y as i32;

                    // 生成高度
                    let height = generator.generate_height(world_x as f64, world_y as f64);
                    data.set_height(x, y, height);

                    // 确定瓦片类型
                    let tile_type =
                        generator.determine_tile_type(height, world_x as f64, world_y as f64);
                    data.set_tile(x, y, tile_type);
                }
            }
        }

        data
    }

    /// 获取区块实体
    pub fn get_chunk_entity(&self, coord: ChunkCoord) -> Option<Entity> {
        self.chunks.get(&coord).copied()
    }

    /// 获取区块
    pub fn get_chunk(&self, coord: ChunkCoord) -> Option<&Entity> {
        self.chunks.get(&coord)
    }

    /// 获取可变区块
    pub fn get_chunk_mut(&mut self, coord: ChunkCoord) -> Option<&mut Entity> {
        self.chunks.get_mut(&coord)
    }

    /// 移除区块
    pub fn remove_chunk(&mut self, coord: ChunkCoord) -> Option<Entity> {
        self.chunks.remove(&coord)
    }

    /// 获取渲染设置
    pub fn render_settings(&self) -> &RenderSettings {
        &self.render_settings
    }

    /// 获取可变渲染设置
    pub fn render_settings_mut(&mut self) -> &mut RenderSettings {
        &mut self.render_settings
    }
}

/// 区块相对方向枚举
/// 用于表示相邻区块的相对位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}
