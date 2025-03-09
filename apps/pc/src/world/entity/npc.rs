use bevy::prelude::*;
use rand::Rng;
use crate::world::entity::{Character, CharacterState};

/// NPC类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NpcType {
    Villager,
    Merchant,
    Guard,
    Enemy,
    Boss,
}

/// NPC组件
#[derive(Component)]
pub struct Npc {
    pub npc_type: NpcType,
    pub dialogue_id: Option<String>,
    pub ai_state: AiState,
    pub patrol_points: Vec<Vec3>,
    pub current_patrol_index: usize,
    pub detection_radius: f32,
    pub aggression: f32,
    pub wander_timer: Timer,
}

/// AI状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiState {
    Idle,
    Patrol,
    Wander,
    Chase,
    Attack,
    Flee,
    Talk,
}

impl Default for Npc {
    fn default() -> Self {
        Self {
            npc_type: NpcType::Villager,
            dialogue_id: None,
            ai_state: AiState::Idle,
            patrol_points: Vec::new(),
            current_patrol_index: 0,
            detection_radius: 100.0,
            aggression: 0.0,
            wander_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        }
    }
}

/// 生成NPC实体
pub fn spawn_npc(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec3,
    npc_type: NpcType,
    name: &str,
) -> Entity {
    // 创建角色实体
    let texture_path = match npc_type {
        NpcType::Villager => "textures/characters/villager.png",
        NpcType::Merchant => "textures/characters/merchant.png",
        NpcType::Guard => "textures/characters/guard.png",
        NpcType::Enemy => "textures/characters/enemy.png",
        NpcType::Boss => "textures/characters/boss.png",
    };
    
    let npc_entity = crate::world::entity::spawn_character(
        commands,
        asset_server,
        position,
        name,
        texture_path,
    );
    
    // 添加NPC组件
    commands.entity(npc_entity).insert(Npc {
        npc_type,
        ..default()
    });
    
    npc_entity
}

/// 更新NPC AI系统
pub fn update_npc_ai(
    mut npc_query: Query<(&mut Npc, &mut Character, &mut Transform)>,
    player_query: Query<(&Character, &Transform), (With<crate::world::entity::Player>, Without<Npc>)>,
    time: Res<Time>,
) {
    let player = player_query.get_single();
    
    for (mut npc, mut character, mut transform) in npc_query.iter_mut() {
        // 更新计时器
        npc.wander_timer.tick(time.delta());
        
        match npc.ai_state {
            AiState::Idle => {
                character.state = CharacterState::Idle;
                
                // 随机切换到游荡状态
                if npc.wander_timer.just_finished() {
                    npc.ai_state = AiState::Wander;
                }
            },
            AiState::Wander => {
                character.state = CharacterState::Walking;
                
                // 随机移动
                if npc.wander_timer.just_finished() {
                    let mut rng = rand::thread_rng();
                    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                    character.direction = Vec2::new(angle.cos(), angle.sin());
                    
                    // 有概率回到空闲状态
                    if rng.gen::<f32>() < 0.3 {
                        npc.ai_state = AiState::Idle;
                    }
                }
                
                // 应用移动
                let movement = character.direction * character.speed * 0.5 * time.delta_seconds();
                transform.translation.x += movement.x;
                transform.translation.y += movement.y;
            },
            AiState::Patrol => {
                if !npc.patrol_points.is_empty() {
                    let target = npc.patrol_points[npc.current_patrol_index];
                    let direction = (target - transform.translation).truncate();
                    
                    if direction.length() < 10.0 {
                        // 到达巡逻点，前往下一个
                        npc.current_patrol_index = (npc.current_patrol_index + 1) % npc.patrol_points.len();
                    } else {
                        // 移向巡逻点
                        character.state = CharacterState::Walking;
                        character.direction = direction.normalize();
                        
                        let movement = character.direction * character.speed * time.delta_seconds();
                        transform.translation.x += movement.x;
                        transform.translation.y += movement.y;
                    }
                }
            },
            AiState::Chase => {
                if let Ok((_, player_transform)) = player {
                    let direction = (player_transform.translation - transform.translation).truncate();
                    
                    if direction.length() < 20.0 {
                        // 足够近，切换到攻击状态
                        npc.ai_state = AiState::Attack;
                    } else if direction.length() > npc.detection_radius {
                        // 超出检测范围，回到游荡状态
                        npc.ai_state = AiState::Wander;
                    } else {
                        // 追逐玩家
                        character.state = CharacterState::Running;
                        character.direction = direction.normalize();
                        
                        let movement = character.direction * character.speed * 1.5 * time.delta_seconds();
                        transform.translation.x += movement.x;
                        transform.translation.y += movement.y;
                    }
                }
            },
            AiState::Attack => {
                character.state = CharacterState::Attacking;
                
                // 攻击完成后回到追逐状态
                if character.state == CharacterState::Attacking {
                    // 这里应该有攻击逻辑
                    npc.ai_state = AiState::Chase;
                }
            },
            AiState::Flee => {
                if let Ok((_, player_transform)) = player {
                    let direction = (transform.translation - player_transform.translation).truncate();
                    
                    if direction.length() > npc.detection_radius * 2.0 {
                        // 已经逃离足够远，回到游荡状态
                        npc.ai_state = AiState::Wander;
                    } else {
                        // 远离玩家
                        character.state = CharacterState::Running;
                        character.direction = direction.normalize();
                        
                        let movement = character.direction * character.speed * 1.2 * time.delta_seconds();
                        transform.translation.x += movement.x;
                        transform.translation.y += movement.y;
                    }
                }
            },
            AiState::Talk => {
                character.state = CharacterState::Idle;
                // 对话逻辑应该在其他系统中处理
            },
        }
        
        // 检测玩家
        if npc.npc_type == NpcType::Enemy || npc.npc_type == NpcType::Boss {
            if let Ok((_, player_transform)) = player {
                let distance = (player_transform.translation - transform.translation).length();
                
                if distance < npc.detection_radius && npc.ai_state != AiState::Chase && npc.ai_state != AiState::Attack {
                    npc.ai_state = AiState::Chase;
                }
            }
        }
    }
} 