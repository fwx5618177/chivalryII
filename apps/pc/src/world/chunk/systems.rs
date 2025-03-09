use super::{ChunkLoaderSystem, ChunkManager};
use crate::world::map::MapManager;
use bevy::prelude::*;

/// 区块系统插件
pub struct ChunkSystemPlugin;

impl Plugin for ChunkSystemPlugin {
    fn build(&self, app: &mut App) {
        // 注册资源
        app.init_resource::<ChunkManager>();

        // 注册系统
        app.add_systems(Startup, setup_chunk_system).add_systems(
            Update,
            (
                // ChunkLoaderSystem::update_player_position,
                ChunkLoaderSystem::process_chunk_loading,
                // ChunkLoaderSystem::update_chunk_visibility,
            )
                .chain(),
        );
    }
}

/// 设置区块系统
fn setup_chunk_system(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    map_manager: Res<MapManager>,
) {
    // 初始化地形生成器
    chunk_manager.initialize_terrain_generator(&map_manager);

    // 设置视图距离
    *chunk_manager = ChunkManager::new(5);

    // 初始化地形生成器
    chunk_manager.initialize_terrain_generator(&map_manager);

    info!("区块系统已初始化");
}
