use bevy::{ecs::component::Component, prelude::Color};

use super::TileType;

/// 瓦片实现
///
/// 负责瓦片的具体实现、渲染和交互逻辑
/// 这里实现了map模块中定义的TileType的具体行为
/// 瓦片渲染数据
#[derive(Component)]
pub struct Render {
    pub color: Color,
    pub z_index: f32,
    pub variant: u8, // 用于选择不同的贴图变体
}

impl Default for Render {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            z_index: 0.0,
            variant: 0,
        }
    }
}

impl Render {
    /// 获取瓦片颜色
    pub fn get_color(&self) -> Color {
        self.color
    }

    /// 获取瓦片Z轴索引
    pub fn get_z_index(&self) -> f32 {
        self.z_index
    }

    /// 获取瓦片变体
    pub fn get_variant(&self) -> u8 {
        self.variant
    }

    pub fn from_tile_type(tile_type: TileType) -> Self {
        match tile_type {
            TileType::Empty => Self {
                color: Color::BLACK,
                z_index: 0.0,
                variant: 0,
            },
            TileType::Ground => Self {
                color: Color::rgb(0.0, 0.5, 0.0),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Wall => Self {
                color: Color::rgb(0.5, 0.5, 0.5),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Water => Self {
                color: Color::rgb(0.0, 0.0, 0.5),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Grass => Self {
                color: Color::rgb(0.0, 0.5, 0.0),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Sand => Self {
                color: Color::rgb(0.8, 0.8, 0.0),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Rock => Self {
                color: Color::rgb(0.5, 0.5, 0.5),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Snow => Self {
                color: Color::rgb(1.0, 1.0, 1.0),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Forest => Self {
                color: Color::rgb(0.0, 0.5, 0.0),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Path => Self {
                color: Color::rgb(0.0, 0.0, 0.0),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Plains => Self {
                color: Color::rgb(0.0, 0.5, 0.0),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Wasteland => Self {
                color: Color::rgb(0.5, 0.5, 0.5),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Bamboo => Self {
                color: Color::rgb(0.0, 0.5, 0.0),
                z_index: 0.0,
                variant: 0,
            },
            TileType::DenseForest => Self {
                color: Color::rgb(0.0, 0.5, 0.0),
                z_index: 0.0,
                variant: 0,
            },
            TileType::Mountain => Self {
                color: Color::rgb(0.5, 0.5, 0.5),
                z_index: 0.0,
                variant: 0,
            },
        }
    }
}
