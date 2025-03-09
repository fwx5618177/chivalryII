pub mod chunk;
/// 世界模块
///
/// 包含地图和区块两个主要子模块，负责游戏世界的生成和管理
///
/// # 设计理念
/// 1. 分层架构：地图模块定义规则，区块模块负责实现
/// 2. 模块化：不同功能独立管理
/// 3. 数据驱动：通过配置文件和参数控制世界生成
pub mod map;

use bevy::prelude::*;

/// 世界系统插件
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // 添加地图系统插件
        app.add_plugins(map::MapSystemPlugin);

        // 添加区块系统插件
        app.add_plugins(chunk::ChunkSystemPlugin);

        info!("世界系统已初始化");
    }
}
