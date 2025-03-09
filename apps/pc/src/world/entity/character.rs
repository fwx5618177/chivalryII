use bevy::prelude::*;
use crate::render::components::{SpriteComponent, AnimationComponent, LayerComponent, RenderLayer};

/// 角色状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterState {
    Idle,
    Walking,
    Running,
    Jumping,
    Falling,
    Attacking,
    Defending,
    Hurt,
    Dead,
}

/// 角色组件
#[derive(Component)]
pub struct Character {
    pub name: String,
    pub state: CharacterState,
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub direction: Vec2,
    pub is_grounded: bool,
    pub can_move: bool,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Character".to_string(),
            state: CharacterState::Idle,
            health: 100.0,
            max_health: 100.0,
            speed: 100.0,
            direction: Vec2::ZERO,
            is_grounded: true,
            can_move: true,
        }
    }
}

/// 生成角色实体
pub fn spawn_character(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec3,
    name: &str,
    texture_path: &str,
) -> Entity {
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(position)),
        Character {
            name: name.to_string(),
            ..default()
        },
        SpriteComponent {
            texture_path: texture_path.to_string(),
            size: Vec2::new(32.0, 64.0),
            offset: Vec2::ZERO,
            flip_x: false,
            flip_y: false,
            color: Color::WHITE,
            visible: true,
        },
        AnimationComponent {
            animation_type: crate::render::components::AnimationType::Sprite,
            current_animation: "idle".to_string(),
            animations: vec![
                "idle".to_string(),
                "walk".to_string(),
                "attack".to_string(),
            ],
            frame_time: std::time::Duration::from_millis(100),
            current_frame: 0,
            total_frames: 4,
            is_playing: true,
            is_looping: true,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
        LayerComponent {
            layer: RenderLayer::Character,
            sub_order: 0,
        },
    )).id()
}

/// 更新角色状态系统
pub fn update_character_state(
    mut query: Query<(&mut Character, &mut AnimationComponent)>,
    time: Res<Time>,
) {
    for (mut character, mut animation) in query.iter_mut() {
        // 根据角色状态更新动画
        let anim_name = match character.state {
            CharacterState::Idle => "idle",
            CharacterState::Walking => "walk",
            CharacterState::Running => "run",
            CharacterState::Attacking => "attack",
            CharacterState::Defending => "defend",
            CharacterState::Hurt => "hurt",
            CharacterState::Dead => "dead",
            _ => "idle",
        };
        
        // 如果状态改变，切换动画
        if animation.current_animation != anim_name {
            animation.current_animation = anim_name.to_string();
            animation.current_frame = 0;
            animation.timer.reset();
        }
    }
} 