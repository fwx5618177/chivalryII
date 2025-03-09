/// 瓦片属性
#[derive(Debug, Clone)]
pub struct Properties {
    pub walkable: bool,     // 是否可行走
    pub blocks_sight: bool, // 是否阻挡视线
    pub movement_cost: f32, // 移动消耗
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            walkable: true,
            blocks_sight: false,
            movement_cost: 1.0,
        }
    }
}
