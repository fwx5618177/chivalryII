use bevy::ecs::component::Component;

/// 瓦片物理属性
#[derive(Component)]
pub struct Physics {
    pub walkable: bool,
    pub blocks_sight: bool,
    pub movement_cost: f32,
}
