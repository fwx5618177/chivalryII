use super::{Chunk, CHUNK_SIZE};
use crate::world::map::MapManager;
use bevy::prelude::*;

/// 2.5D渲染设置
#[derive(Resource)]
pub struct RenderSettings {
    /// 是否启用2.5D效果
    pub enable_2_5d: bool,
    /// 高度缩放因子
    pub height_scale: f32,
    /// 视角倾斜角度（弧度）
    pub tilt_angle: f32,
    /// 阴影强度
    pub shadow_strength: f32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            enable_2_5d: true,
            height_scale: 0.5,
            tilt_angle: 0.6, // 约30度
            shadow_strength: 0.3,
        }
    }
}

/// 计算2.5D渲染中的高度偏移
pub fn calculate_height_offset(height: f32, settings: &RenderSettings) -> Vec2 {
    if !settings.enable_2_5d {
        return Vec2::ZERO;
    }

    let scaled_height = height * settings.height_scale;
    let x_offset = scaled_height * settings.tilt_angle.sin();
    let y_offset = scaled_height * settings.tilt_angle.cos();

    Vec2::new(x_offset, y_offset)
}

/// 根据地图管理器获取高度值
pub fn get_height_from_map(x: i32, y: i32, map_manager: &MapManager) -> f32 {
    map_manager.get_height_at(x, y)
}

/// 为区块中的瓦片应用2.5D效果
pub fn apply_2_5d_effect(
    chunk: &Chunk,
    chunk_entity: Entity,
    map_manager: &MapManager,
    settings: &RenderSettings,
    commands: &mut Commands,
) {
    if let Some(chunk_data) = &chunk.data {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                if let Some(tile_type_value) = chunk_data.get_tile(x, y) {
                    // 计算世界坐标
                    let world_x = chunk.coord.x * CHUNK_SIZE as i32 + x as i32;
                    let world_y = chunk.coord.y * CHUNK_SIZE as i32 + y as i32;

                    // 获取高度
                    let height = get_height_from_map(world_x, world_y, map_manager);

                    // 计算2.5D偏移
                    let offset = calculate_height_offset(height, settings);

                    // 创建瓦片实体并添加到区块
                    // 这里只是示例，实际实现会更复杂
                    let tile_entity = commands
                        .spawn((
                            // 位置组件
                            Transform::from_xyz(
                                x as f32 * 32.0 + offset.x,
                                y as f32 * 32.0 + offset.y,
                                height,
                            ),
                            // 其他组件...
                        ))
                        .id();

                    // 将瓦片实体添加为区块的子实体
                    commands.entity(chunk_entity).add_child(tile_entity);
                }
            }
        }
    }
}

/// 计算相邻瓦片的高度差，用于生成边缘效果
pub fn calculate_height_difference(
    x: i32,
    y: i32,
    neighbor_x: i32,
    neighbor_y: i32,
    map_manager: &MapManager,
) -> f32 {
    let height = map_manager.get_height_at(x, y);
    let neighbor_height = map_manager.get_height_at(neighbor_x, neighbor_y);

    (height - neighbor_height).abs()
}
