use super::{Behavior as NpcBehavior, Dialogue, NpcType};
use bevy::math::IVec2;

use super::super::Quest;

/// NPC配置结构
#[derive(Debug, Clone)]
pub struct Npc {
    /// NPC的唯一标识符
    pub id: String,
    /// NPC类型（商人、守卫、村民等）
    pub npc_type: NpcType,
    /// NPC的初始位置
    pub position: IVec2,
    /// NPC的行为模式
    pub behavior: NpcBehavior,
    /// NPC的对话内容
    pub dialogues: Vec<Dialogue>,
    /// NPC的任务列表
    pub quests: Vec<Quest>,
}

impl Default for Npc {
    fn default() -> Self {
        Self {
            id: String::new(),
            npc_type: NpcType::Villager,
            position: IVec2::ZERO,
            behavior: NpcBehavior {
                alert_range: 5.0,
                interaction_range: 2.0,
            },
            dialogues: Vec::new(),
            quests: Vec::new(),
        }
    }
}
