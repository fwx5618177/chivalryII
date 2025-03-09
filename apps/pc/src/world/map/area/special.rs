use super::area::Area;
use bevy::utils::HashMap;

/// 特殊区域类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpecialAreaType {
    SecretCave,     // 秘密洞穴
    AncientRuins,   // 古代遗迹
    SacredGrove,    // 灵药谷
    BattleGround,   // 古战场
    MartialArena,   // 武道场
    MeditationSpot, // 修炼点
}

/// 特殊区域规则
#[derive(Debug, Clone)]
pub struct SpecialAreaRules {
    /// 特殊区域及其生成配置
    pub areas: HashMap<SpecialAreaType, Area>,
    /// 区域间最小距离
    pub min_distance: f32,
    /// 每个区块的最大特殊区域数量
    pub max_per_chunk: i32,
}

impl Default for SpecialAreaRules {
    fn default() -> Self {
        Self {
            areas: HashMap::new(),
            min_distance: 10.0,
            max_per_chunk: 2,
        }
    }
}
