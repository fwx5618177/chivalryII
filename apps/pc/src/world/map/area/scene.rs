use crate::world::map::EnvironmentRequirements;

use super::super::{assets::AssetItem, npc::Npc, quest::QuestTrigger};
use super::{building::Building, terrain::TerrainCompatibility};
use bevy::math::{IVec2, Rect};
use std::collections::HashMap;

/// 场景类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SceneType {
    Village,     // 村落
    Town,        // 城镇
    City,        // 主城
    Temple,      // 寺庙
    Waterfall,   // 瀑布
    Lake,        // 湖泊
    Forest,      // 森林
    Mountain,    // 山脉
    Cave,        // 洞窟
    BattleField, // 战场
    SecretRealm, // 秘境
}

/// 固定场景结构
#[derive(Debug, Clone)]
pub struct FixedScene {
    /// 场景类型
    pub scene_type: SceneType,
    /// 占用区域
    pub bounds: Rect,
    /// NPC配置
    pub npcs: Vec<Npc>,
    /// 建筑配置
    pub buildings: Vec<Building>,
    /// 特殊物品
    pub items: Vec<AssetItem>,
    /// 任务触发器
    pub quest_triggers: Vec<QuestTrigger>,
}

/// 场景分布规则
#[derive(Debug, Clone)]
pub struct SceneRules {
    /// 场景密度 (0.0-1.0)
    pub density: f32,

    /// 最小场景间距
    pub min_distance: f32,

    /// 场景类型权重
    pub type_weights: HashMap<SceneType, f32>,

    /// 场景环境要求
    pub environment_requirements: HashMap<SceneType, EnvironmentRequirements>,
}

impl Default for SceneRules {
    fn default() -> Self {
        Self {
            density: 0.5,
            min_distance: 10.0,
            type_weights: HashMap::new(),
            environment_requirements: HashMap::new(),
        }
    }
}

/// 场景分布控制
#[derive(Debug, Clone)]
pub struct SceneDistribution {
    /// 场景密度参数
    pub density: f32,
    /// 最小场景间距
    pub min_distance: f32,
    /// 场景类型权重
    pub type_weights: HashMap<SceneType, f32>,
    /// 地形适应性规则
    pub terrain_compatibility: HashMap<SceneType, TerrainCompatibility>,
}
