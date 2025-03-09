use super::{area::TerrainConfig, Climate, MapManager, Vegetation, Water};
use bevy::prelude::*;

/// 地图系统插件
pub struct MapSystemPlugin;

impl Plugin for MapSystemPlugin {
    fn build(&self, app: &mut App) {
        // 注册资源
        app.init_resource::<MapManager>()
            .add_systems(Startup, setup_map_system);
    }
}

/// 设置地图系统
fn setup_map_system(mut commands: Commands, mut map_manager: ResMut<MapManager>) {
    // 设置随机种子
    let seed = rand::random::<u32>();
    *map_manager = MapManager::new(seed);

    // 配置地形
    let terrain_config = TerrainConfig::default();
    map_manager.update_terrain_config(terrain_config);

    // 配置水系
    let water_config = Water::default();
    map_manager.update_water_config(water_config);

    // 配置植被
    let vegetation_config = Vegetation::default();
    map_manager.update_vegetation_config(vegetation_config);

    // 配置气候
    let climate_config = Climate::default();
    map_manager.update_climate_config(climate_config);

    // 启用2.5D效果
    map_manager.set_enable_2_5d(true);
    map_manager.set_height_scale(0.5);

    info!("地图系统已初始化，种子: {}", seed);
}
