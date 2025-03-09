use super::{Properties as TileProperties, TileType};

/// 地图瓦片结构
#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub height: f32,
    pub walkable: bool,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::Empty,
            height: 0.0,
            walkable: true,
        }
    }
}

impl Tile {
    /// 获取瓦片类型对应的属性
    pub fn get_properties(tile_type: TileType) -> TileProperties {
        match tile_type {
            TileType::Empty => TileProperties {
                walkable: true,
                blocks_sight: false,
                movement_cost: 1.0,
            },
            TileType::Ground => TileProperties {
                walkable: true,
                blocks_sight: false,
                movement_cost: 1.0,
            },
            TileType::Wall => TileProperties {
                walkable: false,
                blocks_sight: true,
                movement_cost: 0.0,
            },
            TileType::Water => TileProperties {
                walkable: false,
                blocks_sight: false,
                movement_cost: 5.0,
            },
            TileType::Grass => TileProperties {
                walkable: true,
                blocks_sight: false,
                movement_cost: 1.2,
            },
            TileType::Sand => TileProperties {
                walkable: true,
                blocks_sight: false,
                movement_cost: 1.5,
            },
            TileType::Rock => TileProperties {
                walkable: false,
                blocks_sight: true,
                movement_cost: 0.0,
            },
            TileType::Snow => TileProperties {
                walkable: true,
                blocks_sight: false,
                movement_cost: 2.0,
            },
            TileType::Forest => TileProperties {
                walkable: true,
                blocks_sight: true,
                movement_cost: 1.8,
            },
            TileType::Path => TileProperties {
                walkable: true,
                blocks_sight: false,
                movement_cost: 0.8,
            },
            TileType::Plains => TileProperties {
                walkable: true,
                blocks_sight: false,
                movement_cost: 1.0,
            },
            TileType::Wasteland => TileProperties {
                walkable: true,
                blocks_sight: false,
                movement_cost: 1.0,
            },
            TileType::Bamboo => TileProperties {
                walkable: true,
                blocks_sight: true,
                movement_cost: 1.5,
            },
            TileType::DenseForest => TileProperties {
                walkable: true,
                blocks_sight: true,
                movement_cost: 2.0,
            },
            TileType::Mountain => TileProperties {
                walkable: false,
                blocks_sight: true,
                movement_cost: 0.0,
            },
        }
    }
}
