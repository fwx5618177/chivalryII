use bevy::prelude::Color;

use super::{Physics as TilePhysics, Render as TileRender, TileType};

/// 根据高度值获取瓦片颜色
pub fn height_to_color(height: f32) -> (u8, u8, u8) {
    if height < 0.1 {
        // 深水
        (0, 0, 153)
    } else if height < 0.2 {
        // 浅水
        (0, 102, 204)
    } else if height < 0.3 {
        // 沙滩
        (210, 180, 140)
    } else if height < 0.5 {
        // 平原/草地
        (34, 139, 34)
    } else if height < 0.7 {
        // 丘陵/树林
        (0, 100, 0)
    } else if height < 0.85 {
        // 山地
        (128, 128, 128)
    } else {
        // 雪山
        (255, 255, 255)
    }
}

/// 混合两种颜色
pub fn blend_colors(color1: (u8, u8, u8), color2: (u8, u8, u8), factor: f32) -> (u8, u8, u8) {
    let factor = factor.min(1.0).max(0.0);
    let inverted = 1.0 - factor;

    (
        (color1.0 as f32 * inverted + color2.0 as f32 * factor) as u8,
        (color1.1 as f32 * inverted + color2.1 as f32 * factor) as u8,
        (color1.2 as f32 * inverted + color2.2 as f32 * factor) as u8,
    )
}

/// 获取瓦片类型对应的物理属性
pub fn get_tile_physics(tile_type: TileType) -> TilePhysics {
    match tile_type {
        TileType::Empty => TilePhysics {
            walkable: true,
            blocks_sight: false,
            movement_cost: 1.0,
        },
        TileType::Ground => TilePhysics {
            walkable: true,
            blocks_sight: false,
            movement_cost: 1.0,
        },
        TileType::Wall => TilePhysics {
            walkable: false,
            blocks_sight: true,
            movement_cost: 0.0,
        },
        TileType::Water => TilePhysics {
            walkable: false,
            blocks_sight: false,
            movement_cost: 5.0,
        },
        TileType::Grass => TilePhysics {
            walkable: true,
            blocks_sight: false,
            movement_cost: 1.2,
        },
        TileType::Sand => TilePhysics {
            walkable: true,
            blocks_sight: false,
            movement_cost: 1.5,
        },
        TileType::Rock => TilePhysics {
            walkable: false,
            blocks_sight: true,
            movement_cost: 0.0,
        },
        TileType::Snow => TilePhysics {
            walkable: true,
            blocks_sight: false,
            movement_cost: 2.0,
        },
        TileType::Forest => TilePhysics {
            walkable: true,
            blocks_sight: true,
            movement_cost: 1.8,
        },
        TileType::Path => TilePhysics {
            walkable: true,
            blocks_sight: false,
            movement_cost: 0.8,
        },
        TileType::Plains => TilePhysics {
            walkable: true,
            blocks_sight: false,
            movement_cost: 1.0,
        },
        TileType::Wasteland => TilePhysics {
            walkable: true,
            blocks_sight: false,
            movement_cost: 1.0,
        },
        TileType::Bamboo => TilePhysics {
            walkable: true,
            blocks_sight: true,
            movement_cost: 1.5,
        },
        TileType::DenseForest => TilePhysics {
            walkable: true,
            blocks_sight: true,
            movement_cost: 1.8,
        },
        TileType::Mountain => TilePhysics {
            walkable: false,
            blocks_sight: true,
            movement_cost: 0.0,
        },
    }
}

/// 获取瓦片类型对应的渲染数据
pub fn get_tile_render(tile_type: TileType, height: f32) -> TileRender {
    let (r, g, b) = match tile_type {
        TileType::Empty => (200, 200, 200),
        TileType::Ground => (139, 115, 85),
        TileType::Wall => (105, 105, 105),
        TileType::Water => (30, 144, 255),
        TileType::Grass => (34, 139, 34),
        TileType::Sand => (210, 180, 140),
        TileType::Rock => (128, 128, 128),
        TileType::Snow => (255, 250, 250),
        TileType::Forest => (0, 100, 0),
        TileType::Path => (160, 82, 45),
        TileType::Plains => (107, 142, 35),
        TileType::Wasteland => (205, 133, 63),
        TileType::Bamboo => (0, 100, 0),
        TileType::DenseForest => (0, 100, 0),
        TileType::Mountain => (128, 128, 128),
    };

    // 根据高度调整颜色亮度，模拟光照效果
    let brightness_factor = 0.5 + height * 0.5;
    let adjusted_color = (
        (r as f32 * brightness_factor).min(255.0),
        (g as f32 * brightness_factor).min(255.0),
        (b as f32 * brightness_factor).min(255.0),
        1.0,
    );

    TileRender {
        color: Color::rgba(
            adjusted_color.0,
            adjusted_color.1,
            adjusted_color.2,
            adjusted_color.3,
        ),
        z_index: height,
        variant: (height * 10.0) as u8 % 3, // 使用高度生成变体，增加视觉多样性
    }
}
