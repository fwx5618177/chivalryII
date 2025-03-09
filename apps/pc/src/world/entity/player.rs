use bevy::prelude::*;
use crate::events::input::{GameAction, InputState};
use crate::world::entity::{Character, CharacterState};
use crate::render::camera::CameraController;

/// 玩家组件
#[derive(Component)]
pub struct Player {
    pub experience: u32,
    pub level: u32,
    pub skill_points: u32,
    pub inventory_capacity: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            experience: 0,
            level: 1,
            skill_points: 0,
            inventory_capacity: 20,
        }
    }
}

/// 生成玩家实体
pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec3,
) -> Entity {
    // 创建角色实体
    let player_entity = crate::world::entity::spawn_character(
        commands,
        asset_server,
        position,
        "Player",
        "textures/characters/player.png",
    );
    
    // 添加玩家组件
    commands.entity(player_entity).insert(Player::default());
    
    player_entity
}

/// 处理玩家输入系统
pub fn handle_player_input(
    input_state: Res<InputState>,
    time: Res<Time>,
    mut player_query: Query<(&mut Character, &mut Transform), With<Player>>,
    mut camera_query: Query<&mut CameraController, With<Camera>>,
) {
    if let Ok((mut character, mut transform)) = player_query.get_single_mut() {
        if !character.can_move {
            return;
        }
        
        let mut direction = Vec2::ZERO;
        
        // 处理移动输入
        if input_state.is_action_active(GameAction::MoveForward) {
            direction.y += 1.0;
        }
        if input_state.is_action_active(GameAction::MoveBackward) {
            direction.y -= 1.0;
        }
        if input_state.is_action_active(GameAction::MoveLeft) {
            direction.x -= 1.0;
            character.direction.x = -1.0;
        }
        if input_state.is_action_active(GameAction::MoveRight) {
            direction.x += 1.0;
            character.direction.x = 1.0;
        }
        
        // 归一化方向向量
        if direction != Vec2::ZERO {
            direction = direction.normalize();
            character.state = CharacterState::Walking;
        } else {
            character.state = CharacterState::Idle;
        }
        
        // 应用移动
        let movement = direction * character.speed * time.delta_seconds();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        
        // 更新相机跟随
        if let Ok(mut controller) = camera_query.get_single_mut() {
            controller.target = Some(player_query.single().1.entity());
        }
        
        // 处理攻击输入
        if input_state.is_action_just_pressed(GameAction::Attack) {
            character.state = CharacterState::Attacking;
            // 这里可以添加攻击逻辑
        }
        
        // 处理交互输入
        if input_state.is_action_just_pressed(GameAction::Interact) {
            // 这里可以添加交互逻辑
        }
    }
} 