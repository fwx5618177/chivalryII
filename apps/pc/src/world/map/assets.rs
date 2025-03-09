use super::effect::{AttributeModifier, AttributeType, ComparisonType, EffectTrigger, EffectType};

// 物品类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    None,       // 无效物品
    Consumable, // 消耗品
    Equipment,  // 装备
    Material,   // 材料
    Quest,      // 任务物品
    Artifact,   // 特殊物品
}

// 物品品质
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetItemQuality {
    Common,    // 普通
    Uncommon,  // 稀有
    Rare,      // 珍稀
    Epic,      // 史诗
    Legendary, // 传说
}

// 物品效果
#[derive(Debug, Clone)]
pub struct AssetItemEffect {
    /// 效果类型
    pub effect_type: EffectType,
    /// 触发条件
    pub trigger: EffectTrigger,
    /// 效果值
    pub value: f32,
    /// 属性修改
    pub modifiers: Vec<AttributeModifier>,
}

// 获取条件
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssetItemRequirement {
    /// 属性类型
    pub attribute: AttributeType,
    /// 比较方式
    pub comparison: ComparisonType,
    /// 比较值
    pub value: f32,
}

/// 物品配置
///
/// # 设计思路
/// 1. 定义物品的基本属性
/// 2. 控制物品的生成规则
/// 3. 管理物品的交互效果
#[derive(Debug, Clone)]
pub struct AssetItem {
    /// 物品类型
    pub artifi_type: AssetType,
    /// 物品数量范围
    pub quantity_range: (u32, u32),
    /// 物品品质
    pub quality: AssetItemQuality,
    /// 生成概率
    pub spawn_chance: f32,
    /// 特殊效果
    pub effects: Vec<AssetItemEffect>,
    /// 获取条件
    pub requirements: Vec<AssetItemRequirement>,
}
